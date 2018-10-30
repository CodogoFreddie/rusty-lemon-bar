use std::collections::HashMap;
use std::{sync, thread, time};

extern crate chrono;
use chrono::prelude::*;

extern crate i3ipc;
use self::i3ipc::event::Event;
use self::i3ipc::I3Connection;
use self::i3ipc::I3EventListener;
use self::i3ipc::Subscription;

use super::format::{Color, Format};
use super::{Block, ThreadResponse};

fn clock(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
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

fn i3(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
    // establish connection.
    let mut listener = I3EventListener::connect().unwrap();
    let mut connection = I3Connection::connect().unwrap();

    // subscribe to a couple events.
    let subs = [Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    // handle them
    for event in listener.listen() {
        let i3ipc::reply::Workspaces { workspaces } = connection.get_workspaces().unwrap();

        let mut workspaces_by_output = HashMap::new();

        for w in workspaces {
            use std::collections::hash_map::Entry;

            match workspaces_by_output.entry(w.output.clone()) {
                Entry::Vacant(e) => {
                    e.insert(vec![w]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(w);
                }
            }
        }

        let mut output_blocks = Vec::<String>::new();

        for (output, workspaces) in &workspaces_by_output {
            for workspace in workspaces {
                let formatters = [
                    Format::Foreground(if workspace.visible {
                        Color::Black
                    } else {
                        Color::Blue
                    }),
                    Format::Background(if workspace.visible {
                        Color::Green
                    } else {
                        Color::Black
                    }),
                ];

                let output_formatted = formatters
                    .iter()
                    .fold(format!(" {} ", workspace.name), |acc, f| f.apply(acc));

                output_blocks.push(output_formatted)
            }
        }

        tx.send(
            (ThreadResponse {
                block: Block::I3(0),
                msg: output_blocks.join(""),
            }),
        ).unwrap();
    }
}

type BlockArray = Vec<fn(std::sync::mpsc::Sender<ThreadResponse>)>;

pub fn get_block_renderers() -> BlockArray {
    vec![clock, i3]
}
