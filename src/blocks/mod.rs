use std::collections::HashMap;
use std::time::Duration;
use std::{sync, thread, time};

extern crate systemstat;
use self::systemstat::{Platform, System};

extern crate chrono;
use chrono::prelude::*;

extern crate i3ipc;
use self::i3ipc::I3Connection;
use self::i3ipc::I3EventListener;
use self::i3ipc::Subscription;

use super::format::{Color, Format};
use super::{Block, ThreadResponse};

fn disk(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
    let sys = System::new();

    loop {
        let mounts_info: String = match sys.mounts() {
            Ok(mounts) => mounts
                .iter()
                .filter(|mount| mount.total.as_u64() > 4000000000)
                .map(|mount| format!(" {}: {} ", mount.fs_mounted_on, mount.avail))
                .collect::<Vec<String>>()
                .join(""),
            _ => String::from(""),
        };

        let output_formatted = Format::Foreground(Color::Green).apply(mounts_info);

        tx.send(ThreadResponse {
            block: Block::Disk,
            msg: output_formatted,
        })
        .unwrap();

        thread::sleep(time::Duration::from_millis(10000));
    }
}

fn cpu(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
    let sys = System::new();

    loop {
        let memory = match sys.memory() {
            Ok(mem) => {
                let total = mem.total.as_u64();
                let used = total - mem.free.as_u64();
                let text = format!(" {}/{} ", systemstat::data::ByteSize::b(used), mem.total);

                let formatters = [
                    Format::SwapAt(used as f32 / total as f32),
                    Format::Foreground(Color::Black),
                    Format::Background(Color::Purple),
                ];

                formatters.iter().fold(text, |acc, f| f.apply(acc))
            }
            Err(_) => String::from(""),
        };

        let load = match sys.load_average() {
            Ok(loadavg) => format!(
                "{:.1} {:.1} {:.1}",
                loadavg.one, loadavg.five, loadavg.fifteen
            ),
            Err(_) => String::from(""),
        };

        let output = format!(
            "{} {} ",
            memory,
            Format::Foreground(Color::Blue).apply(load),
        );

        let output_formatted = Format::Background(Color::Black).apply(output);

        tx.send(ThreadResponse {
            block: Block::Cpu,
            msg: output_formatted,
        })
        .unwrap();

        thread::sleep(time::Duration::from_millis(3000));
    }
}

fn battery(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
    let sys = System::new();

    loop {
        let (percentage, remaining_time) = match sys.battery_life() {
            Ok(battery) => (battery.remaining_capacity * 100.0, battery.remaining_time),
            Err(_) => (0.0, Duration::new(0, 0)),
        };

        let is_charging = match sys.on_ac_power() {
            Ok(power) => power,
            Err(_) => false,
        };

        let done_time = Local::now() + chrono::Duration::seconds(remaining_time.as_secs() as i64);

        let output = format!(
            " {} {}% {} ",
            if is_charging { ">" } else { "<" },
            percentage as u8,
            done_time.format("%H:%M"),
        );

        let formatters = [
            Format::SwapAt(percentage / 100.0),
            Format::Foreground(Color::Black),
            Format::Background(if percentage > 40.0 {
                Color::Green
            } else if percentage > 20.0 {
                Color::Orange
            } else {
                Color::Red
            }),
        ];

        let output_formatted = formatters.iter().fold(output, |acc, f| f.apply(acc));

        tx.send(ThreadResponse {
            block: Block::Battery,
            msg: output_formatted,
        })
        .unwrap();

        thread::sleep(time::Duration::from_millis(10000));
    }
}

fn clock(tx: sync::mpsc::Sender<super::ThreadResponse>) -> () {
    loop {
        let date = Local::now();
        let output = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
        let formatters = [
            Format::Foreground(Color::White),
            Format::Background(Color::Black),
        ];

        let output_formatted = formatters.iter().fold(output, |acc, f| f.apply(acc));
        tx.send(ThreadResponse {
            block: Block::Clock,
            msg: output_formatted,
        })
        .unwrap();

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
    for _event in listener.listen() {
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

        for (_output, workspaces) in &workspaces_by_output {
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

        tx.send(ThreadResponse {
            block: Block::I3(0),
            msg: output_blocks.join(""),
        })
        .unwrap();
    }
}

type BlockArray = Vec<fn(sync::mpsc::Sender<ThreadResponse>)>;

pub fn get_block_renderers() -> BlockArray {
    vec![clock, i3, battery, cpu, disk]
}
