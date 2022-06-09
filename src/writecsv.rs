// use chrono::prelude::*;
use csv;
use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    firstname: String,
    lastname: String,
    email: String,
}

pub fn run() {
    let file = "./phone.csv";

    let mut wtr = csv::Writer::from_path(file).expect("File creation failed!");

    // let mut wtr = csv::WriterBuilder::new()
    //     .from_path(file)
    //     .expect("File creation failed!");

    for i in 1..10 {
        let person = Person {
            firstname: format!("First Name{}", i),
            lastname: format!("Last Name{}", i),
            email: format!("email{}@yahoo.com", i),
        };

        wtr.serialize(&person).expect("Writing record failed!");
    }

    println!("Records successully writen to file {}", file);
}
