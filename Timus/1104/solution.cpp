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

int char2digit(char c) {
    int d = -1;
    if ('0' <= c && c <= '9') d = c - '0';
    if ('A' <= c && c <= 'Z') d = c - 'A' + 10;
    assert(0 <= d && d < 36);
    return d;
}

int rem(const string &a, int b, int p) {
    int r = 0;
    for (char c : a) {
        r = r * p + char2digit(c);
        r %= b;
    }
    return r;
}

int main() {
    string s;
    cin >> s;
    int n = s.length();

    int k = 2;
    for (char c : s) k = max(k, char2digit(c) + 1);
    assert(2 <= k && k <= 36);

    for (; k <= 36; k++)
        if (rem(s, k - 1, k) == 0) {
            cout << k << '\n';
            return 0;
        }

    cout << "No solution.\n";
    return 0;
}
