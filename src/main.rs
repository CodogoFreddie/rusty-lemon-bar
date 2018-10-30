use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::{thread, time};

extern crate chrono;
use chrono::prelude::*;

mod format;
use format::{Color, Format};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Block {
    Clock,
}

struct ThreadResponse {
    block: Block,
    msg: String,
}

fn clock(tx: std::sync::mpsc::Sender<ThreadResponse>) -> () {
    loop {
        let date = Local::now();
        let output = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
        let formatters = [
            Format::Foreground(Color::White),
            Format::Background(Color::Black),
        ];

        let output_formatted = formatters.iter().fold(output, |acc, f| f.apply(acc));
        tx.send(
            (ThreadResponse {
                block: Block::Clock,
                msg: output_formatted,
            }),
        ).unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn render_blocks(blocks: &HashMap<Block, String>) -> String {
    let mut acc = Vec::new();

    for screen in [0, 1].iter() {
        acc.push(format!("%{{S{}}}", screen));

        acc.push(String::from("%{c}"));

        acc.push(blocks.get(&Block::Clock).unwrap().to_string());
    }

    return acc.join("");
}

fn main() {
    let blockRenderers = [clock];

    let (tx, rx) = mpsc::channel();

    for &block in blockRenderers.iter() {
        let transmitter = mpsc::Sender::clone(&tx);
        thread::spawn(move || block(transmitter));
    }

    let mut block_responses = HashMap::new();
    for msg in rx {
        let ThreadResponse {
            block: block,
            msg: msg,
        } = msg;
        block_responses.insert(block, msg);

        let output = render_blocks(&block_responses);
        print!("\n%{{S1}}{}", output);

        stdout().flush();
    }
}
