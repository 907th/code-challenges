#include <iostream>
#include <cassert>

int get_fibonacci_partial_sum_naive(long long from, long long to) {
    long long sum = 0;
    long long current = 0;
    long long next  = 1;
    for (long long i = 0; i <= to; ++i) {
        if (i >= from) sum += current;
        long long new_current = next;
        next = next + current;
        current = new_current;
    }
    return sum % 10;
}

// This is the function from previous assignment
int fibonacci_sum_fast(long long n) {
    // By printing the answer for first ~50 numbers we can find the period:
    int period[60] = {
        0, 1, 2, 4, 7, 2, 0, 3, 4, 8, 3, 2, 6, 9, 6, 6, 3, 0, 4, 5, 0, 6,
        7, 4, 2, 7, 0, 8, 9, 8, 8, 7, 6, 4, 1, 6, 8, 5, 4, 0, 5, 6, 2, 9,
        2, 2, 5, 8, 4, 3, 8, 2, 1, 4, 6, 1, 8, 0, 9, 0
    };
    return period[n % 60];
}

int get_fibonacci_partial_sum_fast(long long m, long long n) {
    int sum_before_m = m > 0 ? fibonacci_sum_fast(m - 1) : 0;
    int sum_n = fibonacci_sum_fast(n);
    return (sum_n + 10 - sum_before_m) % 10;
}

void check_solution() {
    for (int m = 1; m <= 50; m++)
        for (int n = m; n <= 50; n++)
            assert(get_fibonacci_partial_sum_fast(m, n) == get_fibonacci_partial_sum_naive(m, n));
}

int main() {
    long long from, to;
    std::cin >> from >> to;
    //std::cout << get_fibonacci_partial_sum_naive(from, to) << endl;
    //check_solution();
    std::cout << get_fibonacci_partial_sum_fast(from, to) << std::endl;
}
