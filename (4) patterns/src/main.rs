#![allow(dead_code, unused)]

use futures_util::{stream::FuturesUnordered, StreamExt};
use tokio::task::JoinHandle;

fn some_func(value: usize) -> Result<u32, u32> {
    if value % 2 == 0 {
        return Err(1);
    }
    Ok(value as u32)
}

fn maybe_func(value: u32) -> Option<u32> {
    if value % 2 == 0 {
        return None;
    }
    Some(value)
}

enum MyType {
    Type1,
    Type2(u32),
    Type3(String),
    // Compiler helps with matching.
    // Type4(f32),
}

fn get_my_type() -> MyType {
    MyType::Type2(2)
}

fn get_my_type_can_fail() -> Result<MyType, u32> {
    Ok(MyType::Type2(2))
}

fn main() -> Result<(), u32> {
    let value = some_func(20)?;

    let result = some_func(23);
    let result = match result {
        Ok(value) => value,
        Err(code) => return Err(code),
    };

    let value = match maybe_func(100) {
        Some(value) => value,
        None => 0,
    };

    match get_my_type() {
        MyType::Type1 => (),
        MyType::Type2(value) => (),
        MyType::Type3(value) => (),
    }

    match get_my_type_can_fail() {
        Ok(MyType::Type1) => println!("Nice"),
        Ok(MyType::Type2(value)) if value < 2 => println!("Noice"),
        _ => println!("Not good"),
    }

    Ok(())
}

async fn cool_pattern() {
    let mut tasks = FuturesUnordered::<JoinHandle<u32>>::new();

    while let Some(Ok((return_value))) = tasks.next().await {
        // do stuff...
    }
}
