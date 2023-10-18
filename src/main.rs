use crate::services::sequential_id::{ObjectClass, Sandflake, TimestampGenerator};

mod services;

fn main() {
    let sandflake = Sandflake::new(1, TimestampGenerator::Default);
    // for i in 0..10 {
    //     let id = snowflake.generate_object_id(ObjectClass::);
    //     println!("Seq {} / Generated ID: {}, {:064b}", i, id, id);
    // }
    let id = sandflake.generate_object_id(ObjectClass::Project);
    println!("{:064b}: Project", id);
    let id = sandflake.generate_object_id(ObjectClass::Task);
    println!("{:064b}: Task", id);
    let id = sandflake.generate_object_id(ObjectClass::User);
    println!("{:064b}: User", id);
    let id = sandflake.generate_object_id(ObjectClass::Comment);
    println!("{:064b}: Comment", id);
    let id = sandflake.generate_object_id(ObjectClass::Download);
    println!("{:064b}: Download", id);
}
