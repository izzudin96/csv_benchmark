extern crate csv;

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "../csv/Motor_Vehicle_Register_API.csv";
    let file: File = File::open(file_path)?;
    let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        for field in record.iter() {

            print!("{} ", field);
        }
        println!();
    }

    Ok(())
}
