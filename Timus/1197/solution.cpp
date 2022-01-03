#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int check(int c, int r) {
    if (0 <= c && c <= 7)
        if (0 <= r && r <= 7)
            return 1;
    return 0;
}

int solve(int c, int r) {
    int res = 0;
    res += check(c - 2, r - 1);
    res += check(c - 2, r + 1);
    res += check(c + 2, r - 1);
    res += check(c + 2, r + 1);
    res += check(c - 1, r - 2);
    res += check(c - 1, r + 2);
    res += check(c + 1, r - 2);
    res += check(c + 1, r + 2);
    return res;
}

int main() {
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        cout << solve(s[0] - 'a', s[1] - '1') << '\n';
    }
    return 0;
}
