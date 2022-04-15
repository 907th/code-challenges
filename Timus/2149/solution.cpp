#include <iostream>
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
    int n;
    char f[100];
    char buf[500];
    gets(buf);
    sscanf(buf, "%d", &n);
    gets(buf);
    for (int i = 0; i < n; i++) {
        f[i] = buf[5 * i] == '<' ? '<' : '>';
        // cout << f[i] << '\n';
    }

    // Case 1: < < < > > >
    int ans1 = 0;
    for (int i = 0; i < n; i++) {
        if (i < n / 2 && f[i] == '>') ans1++;
        if (i >= n / 2 && f[i] == '<') ans1++;
    }

    // Case 2: > > > < < <
    int ans2 = 0;
    for (int i = 0; i < n; i++) {
        if (i < n / 2 && f[i] == '<') ans2++;
        if (i >= n / 2 && f[i] == '>') ans2++;
    }

    // Case 3: < > < > < >
    int ans3 = 0;
    for (int i = 0; i < n; i++) {
        if (i % 2 == 0 && f[i] == '>') ans3++;
        if (i % 2 == 1 && f[i] == '<') ans3++;
    }

    // Case 4: > < > < > <
    int ans4 = 0;
    for (int i = 0; i < n; i++) {
        if (i % 2 == 0 && f[i] == '<') ans4++;
        if (i % 2 == 1 && f[i] == '>') ans4++;
    }

    cout << std::min({ ans1, ans2, ans3, ans4 }) << '\n';

    return 0;
}
