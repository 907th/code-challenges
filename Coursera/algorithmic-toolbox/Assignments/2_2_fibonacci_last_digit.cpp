#include <iostream>
#include <cassert>

int get_fibonacci_last_digit_naive(int n) {
    if (n <= 1) return n;

    int previous = 0;
    int current  = 1;

    for (int i = 0; i < n - 1; ++i) {
        int tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
    }

    return current % 10;
}

int get_fibonacci_last_digit_ok(int n) {
    if (n <= 1) return n;

    int previous = 0;
    int current  = 1;

    for (int i = 0; i < n - 1; ++i) {
        int tmp_previous = previous;
        previous = current;
        current = (tmp_previous + current) % 10;
    }

    return current;
}

void test_solution() {
    assert(get_fibonacci_last_digit_ok(3) == 2);
    assert(get_fibonacci_last_digit_ok(10) == 5);
    assert(get_fibonacci_last_digit_ok(200) == 5);
    for (int n = 0; n < 20; ++n)
        assert(get_fibonacci_last_digit_ok(n) == get_fibonacci_last_digit_naive(n));
}

int main() {
    int n;
    std::cin >> n;
    int c = get_fibonacci_last_digit_naive(n);
    //std::cout << get_fibonacci_last_digit_naive(n) << std::endl;
    //test_solution();
    std::cout << get_fibonacci_last_digit_ok(n) << std::endl;
}
