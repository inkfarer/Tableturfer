use std::collections::HashMap;
use serde::Deserialize;
use crate::game::squares::MapSquareType;
use crate::matrix::Matrix;

static MAPS_JSON: &str = include_str!("maps.json");

#[derive(Deserialize, Clone)]
pub struct TableturfMap {
    pub name: String,
    pub squares: Matrix<MapSquareType>,
}

pub trait MapProvider {
    fn get(&self, map_name: &str) -> Option<TableturfMap>;

    fn exists(&self, map_name: &str) -> bool;
}

pub struct MapProviderImpl {
    maps: HashMap<String, TableturfMap>,
}

impl MapProviderImpl {
    pub fn new() -> Self {
        let map_list: Vec<TableturfMap> = serde_json::from_str(MAPS_JSON).unwrap();

        MapProviderImpl {
            maps: map_list.into_iter()
                .map(|map| (map.name.clone(), map))
                .collect()
        }
    }
}

impl MapProvider for MapProviderImpl {
    fn get(&self, map_name: &str) -> Option<TableturfMap> {
        self.maps.get(map_name).map(|map| map.clone())
    }

    fn exists(&self, map_name: &str) -> bool {
        self.maps.contains_key(map_name)
    }
}

pub const DEFAULT_GAME_MAP: &str = "Square";
