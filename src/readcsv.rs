#![allow(unused)]
// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono::prelude::*;
// use chrono::{Datelike, Timelike, Utc};
use csv;
// use dateparser::parse;
use serde::Deserialize;

// use std::process;
// use std::fs::File;
// use std::error::Error;

// #[derive(Debug, Deserialize)]
#[derive(Deserialize)]
pub struct DataRow {
    date: String,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    #[serde(rename = "Name")]
    name: String,
}

// [derive(Debug, Default)]
#[derive(Default)]
pub struct DataFrame {
    pub rows: Vec<DataRow>,
}

pub fn run() {
    let file = "./all_stocks_5yr.csv";
    let mut rdr = csv::Reader::from_path(file).unwrap();

    let headers = rdr.headers().unwrap().clone();

    println!("{:?}", headers);

    let rows: Result<Vec<_>, _> = rdr.deserialize().collect();
    let rows = rows.unwrap();

    let data = DataFrame { rows };

    for rec in data.rows {
        if !rec.name.is_empty() {
            print!("{:<5} ", rec.name);

            if !rec.date.is_empty() {
                let naive_date = NaiveDate::parse_from_str(&rec.date, "%Y-%m-%d").unwrap();
                print!("Date : {:#?} ", naive_date);
            } else {
                print!(" ");
            }

            if rec.open.is_empty() {
                print!("0.00 ")
            } else {
                let open = rec.open.parse::<f32>().unwrap();
                print!("Open : {:.2} ", open);
            }

            if rec.close.is_empty() {
                print!("0.00 ")
            } else {
                let close = rec.close.parse::<f32>().unwrap();
                print!("Close : {:.2} ", close);
            }

            if rec.high.is_empty() {
                print!("0.00 ")
            } else {
                let high = rec.high.parse::<f32>().unwrap();
                print!("High : {:.2} ", high);
            }

            if rec.volume.is_empty() {
                print!("0 ")
            } else {
                let volume = rec.volume.parse::<i64>().unwrap();
                println!("Volume {:#?}", volume);
            }
        }
    }
}

// pub fn run() {
//     let file = "./all_stocks_5yr.csv";

//     let mut rdr = csv::Reader::from_path(file).unwrap();

//     let headers = rdr.headers().unwrap();

//     println!("{:?}", headers);

//     for result in rdr.records() {
//         match result {
//             Ok(record) => {
//                 // let row = record;

//                 println!("{:?}", &record[0]);
//                 break;
//             }
//             Err(err) => {
//                 println!("Error reading CSV file {:?}", err);
//                 process::exit(1);
//             }
//         }
//     }
// }
