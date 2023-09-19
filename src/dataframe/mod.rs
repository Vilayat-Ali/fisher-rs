use std::{collections::HashMap, fs::File, io::{self, BufRead}};
use crate::series::Series;

/// A data frame in **fisher-rs**, is a Two-Dimensional data structure, portenstitially heterogeneous tabular data structure with labeled 
/// axes rows, and columns.
/// 
/// ## Features of a data frame
/// 
/// - Potentially columns are of different types.
/// - Pandas DataFrame size is mutable.
/// - DataFrame labeled axes (rows and columns).
/// - can perform arithmetic operations on rows and columns on DataFrame.
/// 
#[derive(Debug)]
pub struct DataFrame {
    pub frame: HashMap<String, Vec<String>>,
    pub size: (usize, usize),
}

impl DataFrame {

    // creating a data frame from a csv file
    pub fn from_csv(file_path: &str, _delimiter: Option<&'static str>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut frame: HashMap<String, Vec<String>> = HashMap::new();
        let mut records = Vec::new();

        for line in reader.lines() {
            let record: Vec<String> = line?.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
            records.push(record);
        }

        // initializing the frame
        for cell in 0..records[0].len() {
            frame.insert(records[0][cell].clone(), (1..records.len()).map(|row| records[row][cell].clone()).collect::<Vec<String>>());
        }

        Ok(Self {
            frame,
            size: (0, 0)
        })
    }

    // creating data frame from csv
    pub fn from_json() -> Self {
        Self {
            frame: HashMap::new(),
            size: (0, 0)
        }
    }
}