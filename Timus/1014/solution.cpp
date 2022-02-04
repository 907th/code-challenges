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
#include <cctype>
#include <cassert>

using namespace std;

const int PRIM[4] = { 2, 3, 5, 7 };

string solve_fast(int n) {
    if (n == 0) return to_string(10);
    if (n <= 9) return to_string(n);

    int k[10];
    memset(k, 0, sizeof(k));

    for (int p : PRIM) {
        while (n % p == 0) {
            n /= p;
            k[p]++;
        }
    }

    if (n != 1) {
        return to_string(-1);
    }

    while (k[2] >= 3) {
        k[8]++;
        k[2] -= 3;
    }

    bool keep2 = false;
    if (k[2] > 0 && (k[2] + k[3]) % 2 == 1) {
        keep2 = true;
        k[2]--;
    }

    while (k[2] >= 2) {
        k[4]++;
        k[2] -= 2;
    }

    while (k[2] > 0 && k[3] > 0) {
        k[6]++;
        k[2]--;
        k[3]--;
    }

    while (k[3] >= 2) {
        k[9]++;
        k[3] -= 2;
    }

    if (keep2) k[2]++;

    string ans = "";
    for (int i = 2; i <= 9; i++) {
        for (int j = 0; j < k[i]; j++)
            ans = ans + (char) ('0' + i);
    }
    return ans;
}

string solve_naive(int n) {
    for (int q = 1; q <= 10000000; q++) {
        int qq = q;
        int p = 1;
        while (qq > 0) {
            p *= qq % 10;
            qq /= 10;
        }
        if (p == n) return to_string(q);
    }
    return to_string(-1);
}

void check_solution() {
    for (int n = 0; n < 1 << 30; n++) {
        string fast_ans = solve_fast(n);
        string naive_ans = solve_naive(n);
        if (fast_ans.compare(naive_ans) != 0) {
            cout << n << '\n';
            cout << "naive: " << naive_ans << '\n';
            cout << "fast: " << fast_ans << '\n';
            return;
        }
    }
}

int main() {
    //check_solution();
    int n;
    cin >> n;
    cout << solve_fast(n) << '\n';
    return 0;
}
