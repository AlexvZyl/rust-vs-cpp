#include <cstdint>
#include <vector>


struct Foo {
    int bar;
    double baz;
};


class FooContainer {
public:
    FooContainer(size_t num_elements) { 
        buffer.resize(sizeof(Foo) * num_elements);
    }

    Foo& Get(size_t index) {
        return reinterpret_cast<Foo&>(buffer[sizeof(Foo) + index]);
    }

private:
    std::vector<uint8_t> buffer;
};


int main() {
    auto container = FooContainer(1024);
    auto& first = container.Get(0);
}
