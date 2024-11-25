#![allow(unused_variables)]


fn add_zero(input: &mut Vec<u32>) {
    input.push(0);
}


fn take_and_return(mut input: Vec<u32>) -> Vec<u32> {
    add_zero(&mut input);
    input
}


fn take_and_keep(input: Vec<u32>) {
    // Do stuff...
}


fn main() {
    let _immutable_vec = Vec::<u32>::new();

    let mut mutable_vec = Vec::<u32>::new();
    mutable_vec = take_and_return(mutable_vec);

    // :)
    // take_and_keep(mutable_vec);
    // mutable_vec.push(0);

    // This is fine.
    take_and_keep(mutable_vec.clone());
    mutable_vec.push(0);
}
