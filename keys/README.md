# Keys

Crate for listening to global key events across platforms*.

- *Currently supports MacOS only, with others to come

## TODOs (open issues please)

- [ ] Better represent a modified from a key, since only a subset is viable
- [ ] MacOS: Remove left vs right notion for modifiers.

## Building for non-unix envs

- Utilize [cross](https://github.com/rust-embedded/cross)
- `cargo install cross`
- `cross build --target x86_64-pc-windows-gnu`
## Credits

- Heavily lifted from [rdev](https://github.com/Narsil/rdev)