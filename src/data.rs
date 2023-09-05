extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::Rng;

use serde::Deserialize;
use std::error::Error;

pub mod corpora;
pub mod tlds;

/// JSONDataset represents a generic data structure for storing the parsed JSON. Each JSON taken
/// from Corpora has a `data` field which is an Array of Strings in JSON (= Vec<String> in Rust).
#[derive(Deserialize, Debug)]
struct JSONDataset {
    data: Vec<String>,
}

/// `get_dataset` returns the from the constants defined in `src/corpora/data.rs` and parses them into
/// a `JSONDataset struct.
fn get_dataset(key: &str) -> Result<JSONDataset, Box<dyn Error>> {
    let json_dataset: &str = match key {
        "dinosaur" => corpora::DATA_DINOSAURS,
        "cat" => corpora::DATA_CATS,
        "dog" => corpora::DATA_DOGS,
        "horse" => corpora::DATA_HORSES,
        "fabric" => corpora::DATA_FABRIC,
        "gemstone" => corpora::DATA_GEMSTONE,
        "mood" => corpora::DATA_MOOD,
        "tlds" => tlds::DATA_TLDS,
        _ => "",
    };

    let dataset: JSONDataset = serde_json::from_str(json_dataset)?;

    return Ok(dataset);
}

/// `gen_switch` is a special generator that gets its data in JSON format taken from the [Corpora Project](https://github.com/dariusk/corpora). A copy of the entire Corpora project is included in the `data` directory.
/// Not all data sets are available as of now. See the [src/corpora.rs](https://github.com/kevingimbel/fakedata_generator/blob/master/src/corpora.rs) file for all available sets.
///
/// In addition TLDs are generated from <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>
///
/// Possible input values:
///   - `cat`
///   - `dog`
///   - `horse`
///   - `dinosaur`
///   - `gemstone`
///   - `mood`
///   - `fabric`
///   - `tlds`
///
/// Each of these will return a random word from the list.
///
/// ## Example
/// ```rust
/// use fakedata_generator::data::gen_switch;
/// let horse: String = gen_switch("horse".to_string());
/// let gem: String = gen_switch("gemstone".to_string());
/// // horse = Appaloosa
/// // gem = emerald
/// ```
pub fn gen_switch(name: String) -> String {
    let n: &str = name.as_str();
    let data = get_dataset(n).unwrap().data;

    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if data.len() - 1 > 0 {
        index = rnd.gen_range(0..data.len() - 1);
    }

    return data[index].to_string();
}

// gen_corpora_switch is deprecated and should not be used, for now it is a wrapper around gen_switch()
pub fn gen_corpora_switch(name: String) -> String {
    return gen_switch(name);
}
