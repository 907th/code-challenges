#include <iostream>
#include <cassert>

long long lcm_naive(int a, int b) {
    for (long long l = 1; l <= (long long) a * b; ++l)
        if (l % a == 0 && l % b == 0) return l;
    return (long long) a * b;
}

int gcd_fast(int a, int b) {
    if (b == 0) return a;
    return gcd_fast(b, a % b);
}

long long lcm_fast(int a, int b) {
    return (long long) a * b / gcd_fast(a, b);
}

void check_solution() {
    for (int a = 1; a <= 100; a++)
        for (int b = 1; b <= 100; b++)
            assert(lcm_naive(a, b) == lcm_fast(a, b));
}

int main() {
    int a, b;
    std::cin >> a >> b;
    //std::cout << lcm_naive(a, b) << std::endl;
    //check_solution();
    std::cout << lcm_fast(a, b) << std::endl;
    return 0;
}
