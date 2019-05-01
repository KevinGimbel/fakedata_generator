extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::Rng;

use std::error::Error;
use serde::Deserialize;

pub mod data;

#[derive(Deserialize, Debug)]
struct JSONDataset {
    data: Vec<String>,
}

fn get_dataset(key: &str) -> Result<JSONDataset, Box<Error>> {
    let json_dataset: &str = match key {
        "dinosaur" => data::DATA_DINOSAURS,
        "cat" => data::DATA_CATS,
        "dog" => data::DATA_DOGS,
        "horse" => data::DATA_HORSES,
        "fabric" => data::DATA_FABRIC,
        "gemstone" => data::DATA_GEMSTONE,
        "mood" => data::DATA_MOOD,
        _ => "",
    };

    let dataset: JSONDataset = serde_json::from_str(json_dataset)?;

    return Ok(dataset);
}

// gen_corpora_switch accepts a name and then returns a value from the related file
pub fn gen_corpora_switch(name: String) -> String {
    let n: &str = name.as_str();
    let data = get_dataset(n).unwrap().data;

    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if data.len()-1 > 0 {
        index = rnd.gen_range(0, data.len() - 1);
    }

    return data[index].to_string();
}