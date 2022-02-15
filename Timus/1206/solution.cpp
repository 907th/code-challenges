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

struct BigNum {
    const static int P = 10000;
    const static int N = 25;

    int v[N];

    BigNum() {
        memset(v, 0, sizeof(v));
    }

    BigNum(int n) {
        memset(v, 0, sizeof(v));
        for (int i = 0; i < N && n > 0; i++) {
            v[i] = n % P;
            n /= P;
        }
    }

    BigNum operator *(int b) {
        BigNum c;
        int d = 0;
        for (int i = 0; i < N; i++) {
            d += v[i] * b;
            c.v[i] = d % P;
            d /= P;
        }
        return c;
    }

    void print() {
        int i = N - 1;
        while (i > 0 && v[i] == 0) i--;
        cout << +v[i];
        while (--i >= 0) cout << setw(4) << setfill('0') << +v[i];
        cout << '\n';
    }
};


// Just precalculate first values:
// 1 - 36
// 2 - 1980
// 3 - 108900
// 4 - ...
int main() {
    int k;
    cin >> k;
    BigNum ans(36);
    while (--k > 0) ans = ans * 55;
    ans.print();
    return 0;
}
