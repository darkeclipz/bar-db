use half::f16;

struct Page {
    pub start: i64,
    pub end: i64,
    pub block: Block
}

struct Block {
    pub time: i64,
    pub resolution: u32,
    pub bars: Vec<Bar>
}

struct Bar {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32
}

fn main() {
    println!("CandleDB\r\nHighly efficient storage of candles.\r\n\r\nCQL> ");
    

}
