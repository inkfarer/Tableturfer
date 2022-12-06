use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use msbt::Msbt;
use sarc::{SarcEntry, SarcFile};
use serde::Serialize;
use strum::EnumCount;

#[derive(Eq, PartialEq, Hash, Debug, EnumCount, Serialize)]
enum LangFileType {
    #[serde(rename = "card")]
    CardName,
    #[serde(rename = "map")]
    MapName,
}

impl LangFileType {
    fn from_file_name(file_name: &str) -> Option<Self> {
        if file_name.contains("MiniGameMapName") {
            Some(Self::MapName)
        } else if file_name.contains("MiniGameCardName") {
            Some(Self::CardName)
        } else {
            None
        }
    }
}

fn read_strings() -> Result<HashMap<LangFileType, SarcEntry>, Box<dyn Error>> {
    let file_content = zstd::decode_all(File::open("input/Mals/EUen.Product.110.sarc.zs")?)?;
    let parsed_sarc = SarcFile::read(&file_content).unwrap();

    let files = parsed_sarc.files.into_iter()
        .filter_map(|file| {
            let file_name = file.name.clone()?;
            let file_type = LangFileType::from_file_name(&file_name);
            file_type.map(|file_type| (file_type, file))
        })
        .collect::<HashMap<LangFileType, SarcEntry>>();
    if files.len() != LangFileType::COUNT {
        Err("Couldn't find one or more of the expected files in the provided SARC archive".into())
    } else {
        Ok(files)
    }
}

fn normalize_translation_string(string: &str) -> String {
    string.trim_end_matches('\0').replace('\n', " ")
}

fn parse_sarc_entry(entry: SarcEntry) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let msbt = Msbt::from_reader(Cursor::new(entry.data))?;
    let strings = msbt
        .txt2().expect("MSBT file is missing required data")
        .strings()?;
    let labels = msbt
        .lbl1().expect("MSBT file is missing required data")
        .labels();

    let mut result: HashMap<String, String> = HashMap::new();
    for label in labels.into_iter() {
        let string = strings.get(label.index() as usize);

        if string.is_none() {
            return Err(format!("Failed to find match for key '{}' (nothing found at index {})", label.name(), label.index()).into());
        }

        result.insert(label.name().to_string(), normalize_translation_string(string.unwrap()));
    }

    Ok(result)
}

// todo: other languages
fn main() {
    let out_dir = "../web/lang/game";

    if !Path::new(out_dir).exists() {
        panic!("Couldn't find output directory");
    }

    let file_contents = read_strings().unwrap();
    let parsed_contents: HashMap<LangFileType, HashMap<String, String>> = file_contents.into_iter()
        .map(|(file_type, file)| {
            (file_type, parse_sarc_entry(file).unwrap())
        })
        .collect();
    let serialized_contents = serde_json::to_string(&parsed_contents).expect("Failed to serialize translations");

    fs::write(format!("{}/{}", out_dir, "en.json"), serialized_contents).expect("Failed to write result");
}
