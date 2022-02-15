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

int main() {
    string s;
    cin >> s;
    assert(s[0] > '0' || s.compare("0") == 0);
    int n = s.length();

    string f(s);
    for (int i = 0; i < n - i - 1; i++) f[n - i - 1] = f[i];

    if (f.compare(s) < 0) {
        int i = (n - 1) / 2;
        while (f[i] == '9') {
            f[i] = '0';
            f[n - i - 1] = '0';
            i--;
        }
        f[i]++;
        f[n - i - 1] = f[i];
        assert(f.compare(s) > 0);
    }

    cout << f << '\n';
    return 0;
}
