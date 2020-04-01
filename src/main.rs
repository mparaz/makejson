#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate rand;

extern crate base64;

use std::io::prelude::*;
use std::fs::File;

use rand::prelude::*;

// It would be nice to make this structure configurable.
#[derive(Serialize)]
struct IngestRecord {
    name: String,
    age: u32,
    data: String,
}

fn main() {
    let mut ingest_record = IngestRecord {
        name: String::from("Miguel"),
        age: 0,
        data: String::from(""),
    };

    // Generate random bytes
    let mut rng = rand::thread_rng();

    // [x; N] is a repeat expression
    // let mut random_bytes: [u8; 4096] = [0; 4096];
    let mut random_bytes = [b'\0'; 4096];

    // Needs to a vec for base64?
    // No, it doesn't work with random function.
    // let mut random_bytes: Vec<u8> = vec![b'\0'; 4096];

    // &mut is needed - mutable reference
    // https://doc.rust-lang.org/std/primitive.pointer.html
    //
    // The array size is part of the type.
    // rng.fill(&mut random_bytes);

    // random_bytes[0] = 0;
    //
    // Random bytes are hard in Rust?
    // I wanted to create an array of random bytes and then base64 it.

    // let as_json = serde_json::to_string(&ingest_record).unwrap();

    // It needs to be random or else it will just compress into nothing.


    // Since the data is not random, just fill it up.
    // ingest_record.data = (0..4096).map({|_| "X"}).collect::<String>();

    for n in 0..10 {
        let filename = format!("file{}.json", n);
        let mut f = File::create(filename).expect("unable to create file");
    
        for _ in 0..1000 {
            rng.fill(&mut random_bytes);

            // base64::encode works with vec for larger data. 
            ingest_record.data = base64::encode(&random_bytes.to_vec());

            serde_json::to_writer(&f, &ingest_record).expect("unable to write json");
            ingest_record.age += 1;
            // f.write() returns an amount written, and may not write the
            // entire content. write_all() will.
            f.write_all(b"\n").expect("unable to write newline");
        }
    }
}
