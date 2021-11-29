#include <iostream>
#include <cassert>

int gcd_naive(int a, int b) {
    int current_gcd = 1;
    for (int d = 2; d <= a && d <= b; d++) {
        if (a % d == 0 && b % d == 0) {
            if (d > current_gcd) {
                current_gcd = d;
            }
        }
    }
    return current_gcd;
}

int gcd_fast(int a, int b) {
    if (b == 0) return a;
    return gcd_fast(b, a % b);
}

void check_solution() {
    for (int a = 1; a < 1000; a++)
        for (int b = 1; b < 1000; b++)
            assert(gcd_naive(a, b) == gcd_fast(a, b));
}

int main() {
    int a, b;
    std::cin >> a >> b;
    //std::cout << gcd_naive(a, b) << std::endl;
    //check_solution();
    std::cout << gcd_fast(a, b) << std::endl;
    return 0;
}
