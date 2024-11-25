#![allow(unused_variables, dead_code)]

struct Foo {
    buffer: Vec<u32>,
}

impl Foo {
    pub fn new(buffer: Vec<u32>) -> Self {
        Foo { buffer }
    }
}

fn add_zero(input: &mut Vec<u32>) {
    input.push(0);
}

fn take_and_return(mut input: Vec<u32>) -> Vec<u32> {
    add_zero(&mut input);
    input
}

fn main() {
    let _immutable_vec = Vec::<u32>::new();

    let mut mutable_vec = Vec::<u32>::new();
    mutable_vec = take_and_return(mutable_vec);

    let foo = Foo::new(mutable_vec);
    // mutable_vec.push(0);
}
