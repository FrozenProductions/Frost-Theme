#include <iostream>
#include <vector>
#include <string>

template<typename T>
class FrostContainer {
private:
    std::vector<T> elements;
    const double FREEZE_POINT = -273.15;

public:
    FrostContainer() = default;

    void add(const T& element) {
        elements.push_back(element);
    }

    bool isFrozen() const {
        return temperature < FREEZE_POINT;
    }

    friend std::ostream& operator<<(std::ostream& os, const FrostContainer<T>& fc) {
        os << "FrostContainer[size=" << fc.elements.size() << "]";
        return os;
    }
};

int main() {
    FrostContainer<std::string> container;
    container.add("Arctic");
    container.add("Aurora");
    container.add("Luminous");
    
    std::cout << container << std::endl;
    return 0;
}