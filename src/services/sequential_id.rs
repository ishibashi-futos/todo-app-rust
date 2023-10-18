extern crate chrono;

use chrono::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Sandflake {
    last_timestamp: AtomicU64,
    node_id: u64,
    sequence: AtomicU64,
}

pub enum ObjectClass {
    Unknown = 0b0000,
    Project = 0b0001,
    Task = 0b0010,
    User = 0b0011,
    Comment = 0b0100,
    Download = 0b0101,
}

impl Sandflake {
    pub fn new(node_id: u64) -> Self {
        let mut nid = node_id;
        nid <<= 12;
        Sandflake {
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
            node_id: nid,
        }
    }

    fn current_timestamp(&self) -> u64 {
        let now: DateTime<Utc> = Utc::now();
        now.timestamp_millis() as u64
    }

    pub fn generate_id(&self) -> u64 {
        let mut timestamp = self.current_timestamp();
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);

        if timestamp == last_timestamp {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst) + 1;
            timestamp <<= 22;
            return timestamp | self.node_id | sequence;
        }

        self.last_timestamp.store(timestamp, Ordering::SeqCst);
        self.sequence.store(0, Ordering::SeqCst);

        timestamp <<= 22;
        timestamp | self.node_id
    }

    pub fn generate_object_id(&self, object_class: ObjectClass) -> u64 {
        let mut cls = object_class as u64;
        cls <<= 18;
        let id = self.generate_id();
        println!("{:064b}:id\n{:064b}:class", id, cls);
        id | cls
    }
}