use std::collections::HashMap;
use super::utils::Utils;

pub struct Statistics{}

impl Statistics {
    pub fn print_highest_tiles_statistics(highest_tiles: Vec<u64>) {
        let highest_tile: u64 = Self::get_highest_tile(&highest_tiles);
        println!(
            "Highest_tile: {:?} ({:?})\nAverage: {:?}", 
            highest_tile,
            Utils::freaking_pow_is_not_possible_with_an_u64(highest_tile),
            Self::get_average(&highest_tiles), 
        );
        Self::pp_summary(&highest_tiles);
    }

    fn get_average(tiles: &Vec<u64>) -> f64{
        let mut sum: u64 = 0;
        let mut total_tiles: u64 = 0;
        for tile in tiles.iter() {
            sum += tile;
            total_tiles += 1;
        }
        sum as f64 / total_tiles as f64
    }
    
    fn get_highest_tile(tiles: &Vec<u64>) -> u64 {
        let mut highest_tile: u64 = 0;
        for tile in tiles.iter() {
            if tile > &highest_tile {
                highest_tile = *tile;
            }
        }
        highest_tile
    }

    fn pp_summary(tiles: &Vec<u64>) {
        let mut tiles_map: HashMap<&u64, usize> = HashMap::new();
        for tile in tiles.iter() {
            let tiles_map_tile = tiles_map.entry(tile).or_insert(0);
            * tiles_map_tile += 1;
        }
        println!("------------------------");
        for (tile_value, value_count) in tiles_map {
            println!("{:?}: {:?}", tile_value, value_count);
        }   
    }


}