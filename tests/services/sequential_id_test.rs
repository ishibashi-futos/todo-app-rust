use todo_rs::services::sequential_id::{Sandflake, TimestampGenerator, ObjectClass};

#[test]
#[should_panic]
fn test1() {
    assert_eq!(0, 0)
}