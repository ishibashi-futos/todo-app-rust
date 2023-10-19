use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

type TimestampFn = Box<dyn Fn() -> u64>;

pub struct Sandflake {
    last_timestamp: AtomicU64,
    node_id: u64,
    sequence: AtomicU64,
    current_timestamp: TimestampFn,
}

pub enum ObjectClass {
    #[allow(dead_code)]
    Unknown = 0b0000,
    Project = 0b0001,
    Task = 0b0010,
    User = 0b0011,
    Comment = 0b0100,
    Download = 0b0101,
}

pub enum TimestampGenerator {
    Default,
    #[allow(dead_code)]
    Mock(TimestampFn),
}

const BASE_EPOC_TIME: u64 = 1609459200000; // 2021-01-01 00:00:00 UTCのタイムスタンプを基底として定義。これより前のタイムスタンプは使用しない。

impl Sandflake {
    pub fn new(node_id: u64, timestamp_generator: TimestampGenerator) -> Self {
        let mut nid = node_id;
        nid <<= 12;
        let current_timestamp = match timestamp_generator {
            TimestampGenerator::Default => Box::new(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64 - BASE_EPOC_TIME
            }),
            TimestampGenerator::Mock(mock) => mock,
        };

        Sandflake {
            last_timestamp: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
            node_id: nid,
            current_timestamp,
        }
    }

    // ToDo: テストコードを追加する
    // ToDo: Docコメントを追加する
    pub fn generate_id(&self) -> u64 {
        let mut timestamp = self.current_timestamp.as_ref()();
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);

        if timestamp == last_timestamp {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst) + 1;
            // ToDo: SequenceがOverflowした場合の処理を追加する
            timestamp <<= 22;
            return timestamp | self.node_id | sequence;
        }

        self.last_timestamp.store(timestamp, Ordering::SeqCst);
        self.sequence.store(0, Ordering::SeqCst);

        timestamp <<= 22;
        timestamp | self.node_id
    }

    // ToDo: テストコードを追加する
    // ToDo: Docコメントを追加する
    pub fn generate_object_id(&self, object_class: ObjectClass) -> u64 {
        let mut cls = object_class as u64;
        cls <<= 18;
        let id = self.generate_id();
        id | cls
    }
}
