use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::{thread, time};

extern crate chrono;
use chrono::prelude::*;

mod format;
use format::{Color, Format};

mod blocks;
use blocks::get_block_renderers;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Block {
    Clock,
    I3(u8),
    Network,
    Disk,
}

pub struct ThreadResponse {
    block: Block,
    msg: String,
}

fn render_blocks(blocks: &HashMap<Block, String>) -> String {
    let mut acc = Vec::new();

    for screen in [0, 1].iter() {
        acc.push(format!("%{{S{}}}", screen));

        acc.push(String::from("%{l}"));

        acc.push(match blocks.get(&Block::I3(0)) {
            Some(s) => s.clone(),
            None => String::from(""),
        });

        acc.push(String::from("%{c}"));

        acc.push(match blocks.get(&Block::Clock) {
            Some(s) => s.clone(),
            None => String::from(""),
        });
    }

    return acc.join("");
}

fn main() {
    let blockRenderers = get_block_renderers();

    let (tx, rx) = mpsc::channel();

    for &block in blockRenderers.iter() {
        let transmitter = mpsc::Sender::clone(&tx);
        thread::spawn(move || block(transmitter));
    }

    let mut block_responses = HashMap::new();
    for ThreadResponse { block, msg } in rx {
        block_responses.insert(block, msg);

        let output = render_blocks(&block_responses);

        let stdout = io::stdout(); // get the global stdout entity
        let handle = stdout.lock(); // acquire a lock on it
        let mut handle = io::BufWriter::new(handle); // optional: wrap that handle in a buffer
        writeln!(handle, "{}", output); // add `?` if you care about errors here
    }
}
