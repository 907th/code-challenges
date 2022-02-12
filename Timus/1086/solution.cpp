#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

const int N = 15000;

long long primes[N];

bool is_prime(long long n) {
    for (long long i = 2; i * i <= n; i++)
        if (n % i == 0) return false;
    return true;
}

void calc_primes() {
    primes[0] = 2;
    primes[1] = 3;
    int k = 2;
    long long n = 5;
    while (k < N) {
        if (is_prime(n)) primes[k++] = n;
        n += 2;
    }
}

int main() {
    calc_primes();
    int k;
    cin >> k;
    for (int i = 0; i < k; i++) {
        int n;
        cin >> n;
        cout << primes[n - 1] << '\n';
    }
    return 0;
}
