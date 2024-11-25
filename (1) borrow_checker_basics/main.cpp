#include <iostream>
#include <vector>

struct Foo {
public:
    Foo(std::vector<unsigned>&& buffer) : buffer(std::move(buffer)) {}

private:
    std::vector<unsigned> buffer;
};

void add_zero(std::vector<unsigned> &input) { 
    input.push_back(1); 
}

std::vector<unsigned>&& take_and_return(std::vector<unsigned>&& input) {
    add_zero(input);
    return std::move(input);
}

int main() {
    const auto immutable_vec = std::vector<unsigned int>();

    auto mutable_vec = std::vector<unsigned int>();
    mutable_vec = take_and_return(std::move(mutable_vec));

    // Lekker undefined behavior.
    auto foo = Foo(std::move(mutable_vec));
    mutable_vec.push_back(0);
    std::cout << mutable_vec.size() << std::endl;
}
