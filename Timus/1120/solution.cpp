#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    long long s;
    cin >> s;
    long long n = 2 * (sqrt(s) + 1);
    while (n >= 1) {
        long long x = s - n * (n - 1) / 2;
        if (x > 0 && x % n == 0) {
            cout << (x / n) << ' ' << n << '\n';
            return 0;
        }
        n--;
    }
    throw 1;
    return 0;
}
