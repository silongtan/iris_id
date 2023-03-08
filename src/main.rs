use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use csv::ReaderBuilder;

#[derive(Debug)]
struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/iris.data");
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(contents.as_bytes());

    for result in reader.records() {
        let record = result?;
        let iris = Iris {
            sepal_length: record[0].parse()?,
            sepal_width: record[1].parse()?,
            petal_length: record[2].parse()?,
            petal_width: record[3].parse()?,
            species: record[4].to_string(),
        };
        println!("{:?}", iris);
    }

    Ok(())
}
