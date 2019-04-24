extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::Rng;

use std::error::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Cats {
    cats: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Dogs {
    dogs: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Dinosaurs {
    dinosaurs: Vec<String>,
}


#[derive(Deserialize, Debug)]
struct Horses {
    horses: Vec<String>,
}

fn read_file<P: AsRef<Path>>(path: P)  -> BufReader<File> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return reader;
}

fn read_dinosaurs_from_file<P: AsRef<Path>>(path: P) -> Result<Dinosaurs, Box<Error>> {
    let reader = read_file(path);
    // Read the JSON contents of the file as an instance of `User`.
    let d = serde_json::from_reader(reader)?;
    Ok(d)
}

fn read_horses_from_file<P: AsRef<Path>>(path: P) -> Result<Horses, Box<Error>> {
    let reader = read_file(path);
    // Read the JSON contents of the file as an instance of `User`.
    let d = serde_json::from_reader(reader)?;
    Ok(d)
}


fn read_cats_from_file<P: AsRef<Path>>(path: P) -> Result<Cats, Box<Error>> {
    let reader = read_file(path);
    let d = serde_json::from_reader(reader)?;
    Ok(d)
}

fn read_dogs_from_file<P: AsRef<Path>>(path: P) -> Result<Dogs, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `cat`.
    Ok(u)
}

// gen_corpora_switch accepts a name and then returns a value from the related file
pub fn gen_corpora_switch(name: String) -> String {
    let n: &str = name.as_str();
    let data = match n {
        "dinosaur" => read_dinosaurs_from_file("data/corpora/data/animals/dinosaurs.json").unwrap().dinosaurs,
        "cat" => read_cats_from_file("data/corpora/data/animals/cats.json").unwrap().cats,
        "dog" => read_dogs_from_file("data/corpora/data/animals/dogs.json").unwrap().dogs,
        "horse" => read_horses_from_file("data/corpora/data/animals/horses.json").unwrap().horses,
        _ => vec![String::from("")],
    };

    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if data.len()-1 > 0 {
        index = rnd.gen_range(0, data.len() - 1);
    }

    return data[index].to_string();
}