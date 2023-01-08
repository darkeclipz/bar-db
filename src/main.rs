use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Block {
    pub time: i64,
    pub resolution: u32,
    pub bars: Vec<Bar>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Bar {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32
}

fn main() {

    let mut bars = Vec::new();

    let n_bars = 365 * 24 * 60;
    let file_size = n_bars * 20 + 16;

    for _ in 0..n_bars {
        let bar = Bar {
            open: 1.0,
            high: 2.0,
            low: 3.0,
            close: 4.0,
            volume: 5.0
        };

        bars.push(bar);
    }

    let block = Block { 
        time: 1673187600, 
        resolution: 3600, 
        bars: bars
    };

    let start_encoding = Instant::now();
    let encoded: Vec<u8> = bincode::serialize(&block).unwrap();
    let duration_encoding = start_encoding.elapsed();
    println!("Encoding: {} milliseconds", duration_encoding.as_millis());

    let start_writing = Instant::now();
    let mut file = File::create("block.bin").unwrap();
    file.write_all(&encoded).expect("Error writing data.");
    let duration_writing = start_writing.elapsed();
    println!("Disk write (data): {} Mb", file_size as f32 / 1024.0 / 1024.0);
    println!("Disk write (time): {} milliseconds", duration_writing.as_millis());
    let write_bytes_per_second = (file_size as f32 / 1024.0 / 1024.0) / duration_writing.as_millis() as f32 * 1000.0;
    println!("Disk write: {} Mb/sec", write_bytes_per_second);

}
