extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::Rng;

use std::error::Error;
use serde::Deserialize;

pub mod data;

/// JSONDataset represents a generic data structure for storing the parsed JSON. Each JSON taken
/// from Corpora has a `data` field which is an Array of Strings in JSON (= Vec<String> in Rust).
#[derive(Deserialize, Debug)]
struct JSONDataset {
    data: Vec<String>,
}

/// `get_dataset` returns the from the constants defined in `src/corpora/data.rs` and parses them into
/// a `JSONDataset struct.
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

/// `gen_corpora_switch` is a special generator that gets its data in JSON format taken from the [Corpora Project](https://github.com/dariusk/corpora). A copy of the entire Corpora project is included in the `data` directory.
/// Not all data sets are available as of now. See the [src/corpora/data.rs](https://github.com/kevingimbel/fakedata_generator/blob/master/src/corpora/data.rs) file for all available sets.
///
/// Possible input values:
///   - `cat`
///   - `dog`
///   - `horse`
///   - `dinosaur`
///   - `gemstone`
///   - `mood`
///   - `fabric`
///
/// Each of these will return a random word from the list.
///
/// ## Example
/// ```rust
/// use fakedata_generator::corpora::gen_corpora_switch;
/// let horse: String = gen_corpora_switch("horse".to_string());
/// let gem: String = gen_corpora_switch("gemstone".to_string());
/// // horse = Appaloosa
/// // gem = emerald
/// ```
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