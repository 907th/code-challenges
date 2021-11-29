#include <iostream>
#include <cassert>
#include <vector>

long long get_fibonacci_huge_naive(long long n, long long m) {
    if (n <= 1) return n;
    long long previous = 0;
    long long current  = 1;
    for (long long i = 0; i < n - 1; i++) {
        long long tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
    }
    return current % m;
}

long long get_fibonacci_huge_fast(long long n, long long m) {
    int rem_size = 0;
    std::vector<int> rem;
    rem.push_back(0); rem_size++;
    rem.push_back(1); rem_size++;
    while (1) {
        int num = (rem[rem_size - 2] + rem[rem_size - 1]) % m;
        rem.push_back(num); rem_size++;
        if (rem[rem_size - 2] == 0 && rem[rem_size - 1] == 1) break;
    }
    int period_len = rem_size - 2;
    return rem[n % period_len];
}

void check_solution() {
    for (long long n = 1; n <= 40; n++)
        for (long long m = 2; m <= 1000; m++) {
            int naive_res = get_fibonacci_huge_naive(n, m);
            int fast_res = get_fibonacci_huge_fast(n, m);
            if (naive_res != fast_res) {
                std::cout << "Failed example: " << n << " " << m << std::endl;
                std::cout << "Naive result: " << naive_res << std::endl;
                std::cout << "Fast result: " << fast_res << std::endl;
                throw 1;
            }
        }
}

int main() {
    long long n, m;
    std::cin >> n >> m;
    //std::cout << get_fibonacci_huge_naive(n, m) << std::endl;
    //check_solution();
    std::cout << get_fibonacci_huge_fast(n, m) << std::endl;
    return 0;
}
