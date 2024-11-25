#include <iostream>
#include <memory>
#include <vector>

class Pet {
public:
    // With `virtual`, a vtable gets inseted into the class, increasing its size
    // and tightly couplinfg the class with the idea of inheritance.
    virtual ~Pet() = default;
    virtual void Speak() const noexcept = 0;
};


class Cat : public Pet {
public:
    void Speak() const noexcept override {
        std::cout << "Meow" << std::endl;
    }
};


class Dog : public Pet {
public:
    void Speak() const noexcept override {
        std::cout << "Woof" << std::endl;
    }
};


void speak_from_func(Pet& pet) {
    // This does 3 dereferences:
    // 1. Pointer to the class.
    // 2. Pointer to the vtable.
    // 3. Pointer to the function.
    pet.Speak();
}


std::vector<std::unique_ptr<Pet>> create_pets() {
    std::vector<std::unique_ptr<Pet>> pets;
    pets.reserve(2);
    // These have to be allocated.
    pets.push_back(std::make_unique<Dog>());
    pets.push_back(std::make_unique<Cat>());
    return pets;
}


int main() {
    // Normal construction.
    auto dog = Dog();
    dog.Speak();
    auto cat = Cat();
    cat.Speak();

    // Passing to functions.
    speak_from_func(dog);
    speak_from_func(cat);

    // Iterating over pets.
    auto pets = create_pets();
    for(const auto& pet : pets) {
        // This is 2 dereferences.
        // 1. To vtable.
        // 2. To function.
        pet->Speak();
    }
}
