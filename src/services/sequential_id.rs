use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

const BASE_EPOC_TIME: u64 = 1_609_459_200_000; // 2021-01-01 00:00:00 UTCのタイムスタンプを基底として定義。これより前のタイムスタンプは使用しない。

impl Sandflake {
    pub fn new(node_id: u64, timestamp_generator: TimestampGenerator) -> Self {
        if node_id > 32 {
            panic!(
                "Node ID exceeds the range of 0 to 32, initialization aborted: {}",
                node_id
            );
        }
        let mut nid = node_id;
        nid <<= 12;
        let current_timestamp = match timestamp_generator {
            TimestampGenerator::Default => Box::new(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
                    - BASE_EPOC_TIME
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

    pub fn default(node_id: u64) -> Self {
        Sandflake::new(node_id, TimestampGenerator::Default)
    }

    /// Generate Sequential Id <Sandflake>
    ///
    /// Sandflake is an ID generator that creates unique IDs based on timestamp and combine then with other elements.
    /// The ID is returned as a u64
    ///
    /// # Structure of the ID
    ///
    /// The ID consists of the following components.
    ///
    /// * 0-12 bytes: A sequential ID
    /// * 13-18 bytes: A NodeId with 32 possibilities
    /// * 19-22 bytes: A whitespace for arbitrary codes
    /// * 23-63 bytes: Elapsed time in milliseconds since 2021-01-01 00:00:00.000 UTC
    ///
    /// # note
    ///
    /// If the sequential ID overflows, the ID generator gives up all ID generation, waits 1ms, and generates the next ID.
    ///
    pub fn generate_id(&self) -> u64 {
        let mut timestamp = self.current_timestamp.as_ref()();
        let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);

        if timestamp == last_timestamp {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst) + 1;
            if sequence > 0b111111111111 {
                let duration = Duration::from_millis(1);
                thread::sleep(duration);
                return self.generate_id();
            }
            timestamp <<= 22;
            return timestamp | self.node_id | sequence % 4096;
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
