#include <iostream>
#include <vector>

int fibonacci_sum_squares_naive(long long n) {
    if (n <= 1) return n;
    long long previous = 0;
    long long current = 1;
    long long sum = 1;
    for (long long i = 0; i < n - 1; ++i) {
        long long tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
        sum += current * current;
    }
    return sum % 10;
}

// This function is from assignment 2.5
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

// Form the assignment hint we can conclude that S(n) = F(n) * (F(n - 1) + F(n))
// Use function from the solution to assigment 2.5 for finding last digit of a huge Fibonacci number
int fibonacci_sum_squares_fast(long long n) {
    if (n == 0) return 0;
    int f_n = get_fibonacci_huge_fast(n, 10);
    int f_n_minus_1 = get_fibonacci_huge_fast(n - 1, 10);
    return (f_n * (f_n + f_n_minus_1)) % 10;
}

void check_solution() {
    for (int n = 0; n <= 25; n++) {
        int res_naive = fibonacci_sum_squares_naive(n);
        int res_fast = fibonacci_sum_squares_fast(n);
        if (res_naive != res_fast) {
            std::cout << "n = " << n << std::endl;
            std::cout << "result naive = " << res_naive << std::endl;
            std::cout << "result fast = " << res_fast << std::endl;
            throw 1;
        }
    }
}

int main() {
    long long n = 0;
    std::cin >> n;
    //std::cout << fibonacci_sum_squares_naive(n) << std::endl;
    //check_solution();
    std::cout << fibonacci_sum_squares_fast(n) << std::endl;
}
