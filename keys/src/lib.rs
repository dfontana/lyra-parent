//! Global, Cross-Platform KeyEvent Listener
//!
//! Provides an abstraction over key events emitted on the system. Current support
//!  includes:
//! - MacOS, with Windows/Linux to come
//! - KeyDown, KeyUp reactivity
//! - Async viable*
//!
//! Use the [`Listener`] builder to get started. [`Key`] serves as your proxy into
//! specific keys, while [`Keyset`] represents a grouping of a key and 0+ modifying
//! keys.
//!
//! Example:
//! ```no_run
//! use keys::{Key, Keyset, Listener};
//!
//! Listener::new()
//!   .add_down(Keyset::new(Key::Space, vec![Key::MetaLeft]))
//!   .listen(|e: Keyset| {
//!     // e is the set of keys that was engaged
//!     println!("Pressed {}!", e);
//!   })
//!   .expect("Failed to start listener");
//! ```
//!
//! # Async Support
//!
//! Async is fully in your hands, to illustrate here's a simple Tokio example.
//! The key piece to understand is:
//!
//! - Use channels to communicate between threads
//! - Listening should occur on a separate thread from handling for best results
//!
//! ```no_run
//! let (tx, mut rx) = broadcast::channel(16);
//! task::spawn(async move {
//!   println!("[send] Launching listener");
//!   Listener::new()
//!     .add_up(Keyset::new(Key::Space, vec![Key::MetaLeft]))
//!     .listen(move |e: Keyset| {
//!       let sender = tx.clone();
//!       task::spawn(async move {
//!         match sender.send(e.to_owned()) {
//!           Err(e) => println!("[send] Failed {:?}", e),
//!           Ok(_) => println!("[send] Emitted: {}", e),
//!         }
//!       });
//!     })
//!     .expect("Failed to start listener");
//!   loop {} // Notice we need to keep this thread alive
//! });
//! task::spawn(async move {
//!   println!("[recv] Launching handler");
//!   loop {
//!     match rx.recv().await {
//!       Err(e) => println!("[recv] Failed {:?}", e),
//!       Ok(v) => println!("[recv] {}", v),
//!     }
//!   }
//! });
//! ```

#[macro_use]
extern crate lazy_static;
mod traits;

use crate::traits::{Action, Event};
use std::collections::HashSet;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use crate::macos::listen as _listen;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use crate::windows::listen as _listen;

pub use crate::traits::{Key, Keyset, ListenError};

/// Configures the system.
///
/// Listener is your entrypoint into the system. Upon invoking [`Listener::listen`] you'll
/// be launching the process to look for key events.
pub struct Listener {
  seeking: HashSet<Event>,
}

impl Listener {
  /// Create a new instance of the listener
  pub fn new() -> Self {
    Listener {
      seeking: HashSet::new(),
    }
  }

  /// Register the given [`Keyset`] to be matched against when a key is pressed (down).
  ///
  /// Keep in mind pressing down can trigger multiple events when the key is _held_,
  /// if you're looking for a once-and-done consider using [`Listener::add_down`].
  pub fn add_down(mut self, keys: Keyset) -> Self {
    self.seeking.insert(Event::new(keys, Action::KeyPress));
    self
  }

  /// Register the given [`Keyset`] to be matched against when a key is released (up).
  pub fn add_up(mut self, keys: Keyset) -> Self {
    self.seeking.insert(Event::new(keys, Action::KeyRelease));
    self
  }

  /// Being the listening process.
  ///
  /// Listen does not block the main thread, as it ties into system native event loops
  /// to trigger callbacks. The callback you are providing, however, will only be invoked
  /// when one of the registered [`Keyset`] matches, as discussed in [`Listener::add_up`]
  /// and [`Listener::add_down`].
  pub fn listen<F>(self, cb: F) -> Result<(), ListenError>
  where
    F: 'static + Fn(Keyset) + Send,
  {
    _listen(move |e| {
      self.seeking.contains(&e).then(|| cb(e.keyset.to_owned()));
    })
  }
}
