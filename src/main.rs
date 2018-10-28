use std::{thread, time};
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::collections::HashMap;

extern crate chrono;
use chrono::prelude::*;

mod format;
use format::{Format, Color};

enum Blocks {
    Clock,
}

fn clock(tx: std::sync::mpsc::Sender<(Blocks, String)>) -> () {
    loop {
        let date = Local::now(); 
        let output = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
        let formatters = [
            Format::Foreground(Color::White),
            Format::Background(Color::Black),
        ];

        let output_formatted = formatters.iter().fold( output, |acc, f| { f.apply(acc) } );
        tx.send( (
                Blocks::Clock,
            output_formatted
        )).unwrap();

        thread::sleep( time::Duration::from_millis(1000));
    }
}

fn main() {
    let blockRenderers = [ clock, ]; 

    let (tx, rx) = mpsc::channel();

    for &block in blockRenderers.iter() {
        let transmitter = mpsc::Sender::clone(&tx);
        thread::spawn( move || { block(transmitter) });
    }

    let mut blockResponses = HashMap::new();
    for ( block, resp ) in rx {
        blockResponses.insert(block, resp);
        print!("\n%{{S1}}{}", resp);

        stdout().flush();
    }
}
