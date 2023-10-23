use std::time::Duration;
use todo_rs::services::sequential_id::{ObjectClass, Sandflake, TimestampGenerator};
use tokio::task;
use tokio::time::timeout;

#[test]
fn test_generate_id_is_ok() {
    let sandflake = Sandflake::new(5, TimestampGenerator::Default);

    sandflake.generate_id();
}

#[test]
fn test_generate_id_seq_4096() {
    static mut I: i64 = 0;
    let sandflake = Sandflake::new(
        1,
        TimestampGenerator::Mock(Box::new(|| {
            unsafe {
                I += 1;
                if I > 4095 {
                    return 1_609_459_200_002;
                }
            }
            1_609_459_200_001
        })),
    );

    for _ in 0..4096 {
        sandflake.generate_id();
    }
}

#[tokio::test]
async fn test_generate_id_seq_overflow() {
    static mut I: i64 = 0;
    let duration = Duration::from_millis(1_500);
    let f = Box::new(|| {
        let sandflake = Sandflake::new(
            1,
            TimestampGenerator::Mock(Box::new(|| {
                unsafe {
                    I += 1;
                    if I > 4095 {
                        return 1_609_459_200_002;
                    }
                }
                1_609_459_200_001
            })),
        );
        let mut result: u64 = 0;
        for _ in 0..4096 {
            result = sandflake.generate_id();
        }
        return result;
    });
    let result = timeout(duration, task::spawn_blocking(f)).await;
    match result {
        Ok(Ok(value)) => {
            // オーバーフロー後，シーケンスが0にリセットされる
            let seq = value
                & 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111;
            assert_eq!(seq, 0);
        }
        Ok(Err(_)) => panic!("err"),
        Err(_) => panic!("err"),
    }
}

#[test]
fn test_generate_id_add_node_id() {
    // between 0 and 32;
    let node_ids: [u64; 5] = [0, 1, 10, 31, 32];

    for node_id in node_ids {
        let sandflake = Sandflake::default(node_id);

        let id = sandflake.generate_id();

        let mut actual =
            id & 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011_1111_0000_0000_0000;
        actual >>= 12;
        // panic!("{}, {}, {:64b}", node_id, actual, id);
        assert_eq!(node_id, actual);
    }
}

#[test]
#[should_panic(expected = "Node ID exceeds the range of 0 to 32, initialization aborted: 33")]
fn test_generate_id_should_error_node_id_overflow() {
    let node_id: u64 = 33;

    _ = Sandflake::default(node_id);
}
