#![allow(unused)]

use std::mem::{size_of, transmute};

struct Foo {
    bar: u32,
    baz: f64,
}

struct FooContainer {
    buffer: Vec<u8>,
}

impl FooContainer {
    pub fn new(num_elements: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(size_of::<Foo>() * num_elements, 0);

        FooContainer { buffer }
    }

    pub unsafe fn get_unsafe(&mut self, index: usize) -> &Foo {
        transmute(&mut self.buffer[size_of::<Foo>() * index])
    }

    pub fn get_safe(&mut self, index: usize) -> &Foo {
        unsafe {
            return transmute(&mut self.buffer[size_of::<Foo>() * index]);
        }
    }
}

fn main() {
    let mut container = FooContainer::new(1024);

    unsafe {
        let first = container.get_unsafe(0);
    };

    let first = container.get_safe(0);
}
