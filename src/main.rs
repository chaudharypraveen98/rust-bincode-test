use bincode::{Result, serialize, deserialize};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
}
fn main() {
    
    // Read Person object from JSON file
    let mut file = match File::open("person.json"){
        Ok(data)=> data,
        Err(err)=> {
            eprintln!("Error occurred during file loading: {:?}", err);
            return;
        }
    };
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();
    let persons: Vec<Person> = match serde_json::from_str(&json_str){
        Ok(decoded) => decoded,
        Err(err) => {
            eprintln!("Error occurred during deserialization with JSON: {:?}", err);
            return;
        }
    };

    // Serialize with Bincode
    let start_time = Instant::now();
    let bincode_encoded: Vec<u8> = serialize(&persons).unwrap();
    let bincode_duration = start_time.elapsed();
    println!("Bincode Encoded: {:?}, size: {} bytes, took {} ms", bincode_encoded, bincode_encoded.len(), bincode_duration.as_millis());

    // Deserialize with Bincode
    let start_time = Instant::now();
    let bincode_decoded: Vec<Person> = deserialize(&bincode_encoded[..]).unwrap();
    let bincode_duration = start_time.elapsed();
    println!("Bincode Decoded: {:?}, took {} ms", bincode_decoded, bincode_duration.as_millis());

    // Serialize with JSON
    let start_time = Instant::now();
    let json_encoded = serde_json::to_string(&persons).unwrap();
    let json_duration = start_time.elapsed();
    println!("JSON Encoded: {}, size: {} bytes, took {} ms", json_encoded, json_encoded.len(), json_duration.as_millis());

    // Deserialize with JSON
    let start_time = Instant::now();
    let json_decoded: Vec<Person> = serde_json::from_str(&json_encoded).unwrap();
    let json_duration = start_time.elapsed();
    println!("JSON Decoded: {:?}, took {} ms", json_decoded, json_duration.as_millis());

    // Compare results
    assert_eq!(bincode_decoded, json_decoded);

    // Compare size and encoding/decoding time
    let bincode_size = bincode_encoded.len();
    let json_size = json_encoded.len();
    println!("Bincode size: {} bytes, took {} ms", bincode_size, bincode_duration.as_millis());
    println!("JSON size: {} bytes, took {} ms", json_size, json_duration.as_millis());
    if bincode_size < json_size {
        println!("Bincode is faster and more compact.");
    } else if bincode_size > json_size {
        println!("JSON is faster and more compact.");
    } else {
        println!("Bincode and JSON are equally fast and compact.");
    }

    // generator sample

    // let people: Vec<Person> = (0..255)
    //     .map(|i| Person {
    //         name: format!("Person {}", i),
    //         age: i,
    //         is_student: i % 2 == 0,
    //     })
    //     .collect();

    // let json = serde_json::to_string(&people).unwrap();

    // let mut file = File::create("person.json").unwrap();
    // file.write_all(json.as_bytes()).unwrap();
}