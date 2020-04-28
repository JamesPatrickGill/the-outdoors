# the-outdoors

This project is a game using the amethyst game engine for the OLC game jam, albeit unfinished I am using the project to get familiar with Rust and ECS.

## How to run

To run the game, run the following command, which defaults to the `metal` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` to do so make sure you add the necessary features into the cargo.toml ( I excluded them from mine as VSCode was attempting to background build and failing when it couldn't find vulcan):

```toml
[features]
default = ["metal"]
empty = ["amethyst/empty"]
metal = ["amethyst/metal"]
vulkan = ["amethyst/vulkan"]
```

and use the command:

```bash
cargo run --no-default-features --features "vulkan"
```
