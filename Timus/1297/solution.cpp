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

bool palindrome(const char *s, int n) {
    for (int i = 0; i < n / 2; i++)
        if (s[i] != s[n - i - 1]) return false;
    return true;
}

int main() {
    const int N = 1001;
    int n;
    char s[N];

    gets(s);
    n = strlen(s);

    int l, r;
    l = r = 0;
    for (int i = 0; i < n; i++)
        for (int j = n; j - i + 1 > r - l + 1; j--)
            if (palindrome(s + i, j - i + 1)) {
                l = i;
                r = j;
            }

    for (int i = l; i <= r; i++) cout << s[i];
    cout << '\n';
    return 0;
}
