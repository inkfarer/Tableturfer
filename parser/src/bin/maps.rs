use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

#[derive(Deserialize, Serialize)]
struct TableturfMap {
    name: String,
    squares: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct MapValidationError {
    map_name: String,
    message: String,
}

impl MapValidationError {
    fn new(map: TableturfMap, message: &str) -> Self {
        Self {
            map_name: map.name,
            message: message.to_string(),
        }
    }
}

impl Display for MapValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error in map {}: {}", self.map_name, self.message))
    }
}

impl Error for MapValidationError {}

fn parse_verify_files(files: Vec<String>) -> Result<Vec<TableturfMap>, Box<dyn Error>> {
    let mut result = Vec::new();

    for file in files.into_iter() {
        let map: TableturfMap = serde_json::from_str(&file)?;

        let first_row_len = map.squares[0].len();
        if map.squares.iter().any(|row| row.len() != first_row_len) {
            return Err(Box::new(MapValidationError::new(map, "Rows are not of equal size")));
        } else if map.squares.iter().any(|row| row.iter().any(|square| square < &0 || square > &3)) {
            return Err(Box::new(MapValidationError::new(map, "Couldn't recognize some squares")));
        }

        result.push(map);
    }

    Ok(result)
}

fn read_files() -> Result<Vec<String>, Box<dyn Error>> {
    let maps_dir_contents = fs::read_dir("input/maps")?;
    let map_file_paths: Vec<String> = maps_dir_contents
        .filter_map(|entry| {
            let path = match entry {
                Ok(entry) => {
                    let path = entry.path();
                    path.to_str().map(|str| str.to_string())
                },
                Err(_) => None,
            };

            if let Some(path) = path {
                if path.ends_with(".json") {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut result = Vec::new();
    for path in map_file_paths.into_iter() {
        result.push(fs::read_to_string(path)?);
    }
    Ok(result)
}

fn main() {
    let paths = tableturfer_data_parser::verify_paths("../server/src/game/", "../web/assets/", "maps.json");

    // todo: maybe the resulting format can be compacted further by finding repeating patterns and encoding them?
    println!("Reading maps");
    let file_contents = read_files().unwrap();
    let maps = parse_verify_files(file_contents).unwrap();

    println!("Writing result");
    tableturfer_data_parser::write_string(paths, serde_json::to_string(&maps).unwrap());
    println!("Done!");
}
