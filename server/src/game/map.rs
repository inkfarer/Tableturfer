use std::collections::HashMap;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use crate::game::squares::MapSquareType;
use crate::matrix::Matrix;

static MAPS_JSON: &str = include_str!("maps.json");

pub const RANDOM_MAP_NAME: &str = "random";
pub const DEFAULT_GAME_MAP: &str = "Rectangle";

#[derive(Deserialize, Clone)]
pub struct TableturfMap {
    pub name: String,
    pub squares: Matrix<MapSquareType>,
}

pub trait MapProvider {
    fn get(&self, map_name: &str) -> Option<TableturfMap>;

    fn exists(&self, map_name: &str) -> bool;

    fn pick_random(&self) -> TableturfMap;
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
        match map_name {
            RANDOM_MAP_NAME => Some(self.pick_random()),
            _ => self.maps.get(map_name).cloned()
        }
    }

    fn exists(&self, map_name: &str) -> bool {
        map_name.eq(RANDOM_MAP_NAME) || self.maps.contains_key(map_name)
    }

    fn pick_random(&self) -> TableturfMap {
        let mut rng = rand::thread_rng();
        self.maps.iter().choose(&mut rng).expect("No maps are available?").1.clone()
    }
}
