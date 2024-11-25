#include <vector>


void add_zero(std::vector<unsigned>& input) {
    input.push_back(1);
}


std::vector<unsigned>&& take_and_return(std::vector<unsigned>&& input) {
    add_zero(input);
    return std::move(input);
}


void take_and_keep(std::vector<unsigned>&& input) {
    // Do stuff...
}


int main() {
    const auto immutable_vec = std::vector<unsigned int>();

    auto mutable_vec = std::vector<unsigned int>();
    mutable_vec = take_and_return(std::move(mutable_vec));

    // :(
    take_and_keep(std::move(mutable_vec));
    mutable_vec.push_back(0);
}
