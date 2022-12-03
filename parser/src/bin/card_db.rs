use std::error::Error;
use std::fs;
use std::fs::File;
use std::borrow::Borrow;
use std::collections::HashSet;
use byml::Byml;
use serde::Serialize;

const CARD_FILE_NAME: &str = "cards.json";
const RSDB_DIR: &str = "input/RSDB";
const CARD_GRID_SIZE: usize = 8;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct TableturfCard {
    category: String,
    name: String,
    number: i32,
    rarity: u8,
    season: i32,
    special_cost: i32,
    squares: Vec<Vec<u8>>,
}

impl TryFrom<&Byml> for TableturfCard {
    type Error = Box<dyn Error>;

    fn try_from(value: &Byml) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            squares: Self::try_parse_squares(value)?,
            category: value["Category"].as_string()?.to_owned(),
            name: value["Name"].as_string()?.to_owned(),
            number: value["Number"].as_int()?,
            rarity: match value["Rarity"].as_string()?.borrow() {
                "Common" => 0,
                "Rare" => 1,
                "Fresh" => 2,
                other => panic!("Unknown card rarity value {}", other),
            },
            season: value["Season"].as_int()?,
            special_cost: value["SpecialCost"].as_int()?,
        })
    }
}

impl TableturfCard {
    fn try_parse_squares(value: &Byml) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let parsed_squares = value["Square"].as_array()?.iter()
            .map(|item| {
                match item.as_string()?.borrow() {
                    "Empty" => Ok(0),
                    "Fill" => Ok(1),
                    "Special" => Ok(2),
                    other => Err(format!("Unknown card square {}", other).into())
                }
            })
            .collect::<Result<Vec<u8>, Box<dyn Error>>>()?;

        let chunked_squares: Vec<Vec<u8>> = parsed_squares
            .chunks(CARD_GRID_SIZE)
            .into_iter()
            .map(|chunk| chunk.to_vec())
            .rev()
            .collect();
        if chunked_squares.len() != CARD_GRID_SIZE
            || chunked_squares.iter().any(|row| row.len() != CARD_GRID_SIZE)
        {
            return Err(format!("Unexpected square grid size (expected {}x{})", CARD_GRID_SIZE, CARD_GRID_SIZE).into());
        }

        Ok(Self::remove_empty_rows_and_cols(chunked_squares))
    }

    // note: could theoretically be broken if a card is released with a fully empty row of squares
    // in the middle of the card; the card would be imported incorrectly in such a case.
    fn remove_empty_rows_and_cols(value: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut empty_columns: HashSet<usize> = HashSet::new();
        for col_index in 0..value[0].len() {
            if value.iter().all(|row| row[col_index] == 0) {
                empty_columns.insert(col_index);
            }
        }

        value.into_iter()
            .filter(|row| !row.iter().all(|square| square == &0))
            .map(|row| {
                row.into_iter().enumerate()
                    .filter(|(col_index, _)| !empty_columns.contains(col_index))
                    .map(|(_, square)| square)
                    .collect()
            })
            .collect()
    }
}

fn find_card_info_file() -> Result<String, Box<dyn Error>> {
    let rsdb_dir_contents = fs::read_dir(RSDB_DIR)?;
    rsdb_dir_contents
        .filter_map(|entry| {
            match entry {
                Ok(entry) => entry.file_name().into_string().ok(),
                Err(_) => None
            }
        })
        .find(|name| name.to_lowercase().starts_with("minigamecardinfo"))
        .map_or_else(
            || Err("Could not find card info file".into()),
            |name| Ok(format!("{}/{}", RSDB_DIR, name)))
}

fn parse_card_info() -> Result<Vec<TableturfCard>, Box<dyn Error>> {
    let card_info_file = find_card_info_file()?;
    let card_info = Byml::from_binary(&zstd::decode_all(File::open(card_info_file)?)?)?;
    let mut result: Vec<TableturfCard> = Vec::new();

    for card in card_info.as_array()?.iter() {
        result.push(card.try_into()?)
    }

    Ok(result)
}

fn main() {
    let paths = tableturfer_data_parser::verify_paths("../server/src/game/", "../web/assets/", CARD_FILE_NAME);

    println!("Reading card data");
    let card_info = parse_card_info().expect("Failed to parse card data");
    let card_info_json = serde_json::to_string(&card_info).expect("Failed to convert card data to JSON");

    println!("Writing result to file");
    tableturfer_data_parser::write_string(paths, card_info_json);
    println!("Done!");
}
