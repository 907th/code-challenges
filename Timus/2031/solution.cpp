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

int reversed_digit(int d) {
    assert(0 <= d && d <= 9);
    if (d == 0 || d == 1 || d == 8) return d;
    else if (d == 6) return 9;
    else if (d == 9) return 6;
    else return -1;
}

int reversed_number(int n) {
    assert(1 <= n && n <= 99);
    int d0 = reversed_digit(n % 10);
    int d1 = reversed_digit(n / 10);
    return (d0 == -1 || d1 == -1) ? -1 : d0 * 10 + d1;
}

int main() {
    int n;
    cin >> n;
    int len = 0;
    for (int i = 1; i <= 99; i++) {
        if (reversed_number(i) != -1) {
            len++;
            if (len == n) {
                for (int j = i; j >= i - n + 1; j--) {
                    cout << setw(2) << setfill('0') << reversed_number(j);
                    cout << (j > i - n + 1 ? ' ' : '\n');
                }
                return 0;
            }
        } else {
            len = 0;
        }
    }

    cout << "Glupenky Pierre\n";
    return 0;
}
