# tableturfer-data-parser

Parses Splatoon game data and prepares it for use by Tableturfer.

## Setup

Before it can be used, this project requires some files from a dumped copy of Splatoon 3.  
After adding the required Splatoon data to the `input` directory, the project folder should contain the following:
```
.
├── input
│   ├── Mals
│   │   ├── EUen.Product[...].sarc.zs
│   │   └── [Lang].Product.[Version].sarc.zs
│   ├── maps
│   │   └── ... (These files are created manually.)
│   └── RSDB
│       └── MiniGameCardInfo[...].byml.zs
├── lib
├── src
└── ...
```

## Modules

### card_db

`cargo run --bin card_db`

Parses the game's card database (`RSDB/MiniGameCardInfo[...].byml.zs`) and saves the resulting JSON into the appropriate
source directories.

### maps

`cargo run --bin maps`

Parses and compresses maps (`maps/*.json`) into a single JSON file. 

### lang

`cargo run --bin lang`

Parses translation files (`Mals/*.sarc.zs`) and places them in the correct location.
