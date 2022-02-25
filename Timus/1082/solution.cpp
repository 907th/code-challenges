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

const int N = 1000;

int a[N];
int b[N];

int q(int l, int r) {
    if (l >= r)
        return 0;

    int m;
    int c = 0;
    int x = a[l];
    int i = l - 1;
    int j = r + 1;
    while (true)
    {
        do
        {
            --j;
            ++c;
        }
        while (a[j] > x);

        do
        {
            ++i;
            ++c;
        }
        while (a[i] < x);

        if (i < j)
        {
            int t = a[i];
            a[i] = a[j];
            a[j] = t;
        }
        else
        {
            m = j;
            break;
        }
    }

    return c + q(l, m) + q(m + 1, r);
}

void bruteforce() {
    for (int i = 1; i <= N; i++) {
        for (int j = 0; j < i; j++) b[j] = j + 1;
        while (1) {
            memcpy(a, b, sizeof(b));
            if (q(0, i - 1) == (i * i + 3 * i - 4) / 2) {
                cout << i << ':';
                for (int j = 0; j < i; j++) cout << ' ' << b[j];
                cout << '\n';
                break;
            }
            if (!next_permutation(b, b + i)) break;
        }
    }
}

int main() {
    // bruteforce();
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        cout << (i + 1);
        cout << (i < n - 1 ? ' ' : '\n');
    }
    return 0;
}
