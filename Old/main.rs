#[macro_use]
extern crate log;
mod network;
mod node;
mod helpers;

use log::{LogLevelFilter, LogRecord, LogLevel, LogMetadata};
use node::{Node, Event, NodeConfig, EVENT_ON_CONNECTION_OPEN, EVENT_NODE_INIT};
use std::env;
use helpers::Path;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

fn send_test_event(node: &mut Node, value: u64, buff_x: Arc<Vec<Vec<u8>>>) {
    let mut e = Event::default();
    e.path = Path::new();
    e.name = String::from("test");
    e.target = String::from("tree2");
    e.path.mul(value * value);
    e.data = buff_x[0].clone();
    e.from = node.current_value;
    node.emit(e);
}

fn main() {

    let args: Vec<_> = env::args().collect();

    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    });

    let mut buff_x: Vec<u8> = vec![];
    let mut f = File::open("/Users/tigran/Downloads/out.mp4").unwrap();
    let _ = f.read_to_end(&mut buff_x);
    let cc = Arc::new(vec![buff_x]);

    if args[1] == String::from("tree1") {
        let mut node = Node::new(2);
        let conf = NodeConfig {
            tcp_address: String::from("0.0.0.0:8888"),
            concurrency: 2
        };

        node.on(EVENT_NODE_INIT, Box::new(|event: &Event, _:&mut Node| -> bool {
            println!("Node INIT {:?}", event.target);
            true
        }));

        node.on(EVENT_ON_CONNECTION_OPEN, Box::new(|event: &Event, _:&mut Node| -> bool {
            println!("Connected To -> {:?}", event.from);
            true
        }));

        node.on("test", Box::new(move |event: &Event, node:&mut Node| -> bool {
            // println!("Test Event -> {:?}", event.data.len());
            send_test_event(node, event.from, cc.clone());
            true
        }));

        node.start(conf);
    } else {
        let mut node = Node::new(3);
        let conf = NodeConfig {
            tcp_address: String::from("0.0.0.0:8859"),
            concurrency: 2
        };

        node.on(EVENT_NODE_INIT, Box::new(|event: &Event, node:&mut Node| -> bool {
            println!("Node INIT {:?}", event.target);
            node.tcp_connect("127.0.0.1:8888");
            true
        }));

        let second = cc.clone();

        node.on(EVENT_ON_CONNECTION_OPEN, Box::new(move |event: &Event, node:&mut Node| -> bool {
            println!("Connected To -> {:?}", event.from);
            send_test_event(node, 2, cc.clone());
            true
        }));


        node.on("test", Box::new(move |event: &Event, node:&mut Node| -> bool {
            println!("Test Event -> {:?}", event.data.len());
            send_test_event(node ,2, second.clone());
            true
        }));

        node.start(conf);
    }

    print!("{:?}", "New Implementation");
}