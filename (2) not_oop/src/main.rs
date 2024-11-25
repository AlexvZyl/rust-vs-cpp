#![allow(dead_code)]

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait Voice {
    fn speak(&self);
}

// Dog.

struct Dog;

impl Dog {
    pub fn new() -> Self {
        Dog {}
    }
}

impl Voice for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

// Cat.

struct Cat;

impl Cat {
    pub fn new() -> Self {
        Cat {}
    }
}

impl Voice for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

#[enum_dispatch(Voice)]
enum Pet {
    Dog(Dog),
    Cat(Cat),
}

fn get_pet() -> Pet {
    Pet::Dog(Dog::new())
}

fn speak_from_func(speaker: &dyn Voice) {
    // `&dyn Voice` is a wide pointer:
    // 1. Pointer to data
    // 2. Pointer to vtable
    //
    // Also has to do 3 dereferences (if the data is required), but 1. and 2.
    // can be dereferenced simultaneously.
    speaker.speak();
}

fn create_pets() -> Vec<Pet> {
    let mut pets = Vec::with_capacity(2);
    // No extra allocations!
    pets.push(Pet::Dog(Dog::new()));
    pets.push(Pet::Cat(Cat::new()));
    pets
}

fn main() {
    // Normal construction.
    let dog = Dog::new();
    dog.speak();
    let cat = Cat::new();
    cat.speak();

    // Passing to functions.
    speak_from_func(&dog);
    speak_from_func(&cat);

    // Enums.
    let pet = get_pet();
    match pet {
        Pet::Cat(cat) => cat.speak(),
        Pet::Dog(dog) => dog.speak(),
    };
    // pet.speak();

    // Iterating over pets.
    let pets = create_pets();
    for pet in pets.iter() {
        // No deref, only switch.
        pet.speak();
    }
}
