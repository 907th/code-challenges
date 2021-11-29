#include <iostream>
#include <cassert>

int fibonacci_sum_naive(long long n) {
    if (n <= 1) return n;

    long long previous = 0;
    long long current  = 1;
    long long sum      = 1;
    //std::cout << (sum % 10) << " ";

    for (long long i = 2; i <= n; i++) {
        long long tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
        sum += current;
        //std::cout << (sum % 10) << " ";
    }
    //std::cout << std::endl;

    return sum % 10;
}

int fibonacci_sum_fast(long long n) {
    // By printing the answer for first ~50 numbers we can find the period:
    int period[60] = {
        0, 1, 2, 4, 7, 2, 0, 3, 4, 8, 3, 2, 6, 9, 6, 6, 3, 0, 4, 5, 0, 6,
        7, 4, 2, 7, 0, 8, 9, 8, 8, 7, 6, 4, 1, 6, 8, 5, 4, 0, 5, 6, 2, 9,
        2, 2, 5, 8, 4, 3, 8, 2, 1, 4, 6, 1, 8, 0, 9, 0
    };
    return period[n % 60];
}

int main() {
    long long n = 0;
    std::cin >> n;
    //std::cout << fibonacci_sum_naive(n) << std::endl;
    std::cout << fibonacci_sum_fast(n) << std::endl;
}
