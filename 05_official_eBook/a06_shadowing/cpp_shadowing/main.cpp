#include <iostream>

int main() {
  
    int x = 5;

    int x = x + 1;
    {
        int x = x * 2;
        std::cout << "The value of x in the inner scope is" << x << '\n';
    }

    std::cout << "The value of x is " << x << '\n';
}
