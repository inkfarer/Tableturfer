# tableturfer-data-parser

Parses Splatoon game data and prepares it for use by Tableturfer.

## Usage

- Add the required Splatoon data to the `input` directory. By the end, you should have the following directory structure:
    ```
    .
    ├── input
    │   └── RSDB
    │       └── MiniGameCardInfo[...].byml.zs
    ├── lib
    ├── src
    └── ...
    ```
- Run the program: `cargo run`
