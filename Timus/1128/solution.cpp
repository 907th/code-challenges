#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

// Solution:
//   Keep two sets of children with desired property.
//   Take a next child and append it to one of the sets, so that property remains.
// Algo for chosing a set for the next child:
//   There can be at most three children in existing sets connected to the next one.
//   These children may either belong to one set or two different sets. Choose a set which
//   contains at most 1 connected child. There always exists such a set!
int main() {
    const int N = 7500;
    int n;
    int ecount[N];
    int enemies[N][5];
    cin >> n;
    for (int i = 0; i < n; i++) {
        cin >> ecount[i];
        for (int j = 0; j < ecount[i]; j++) {
            cin >> enemies[i][j];
            enemies[i][j]--;
        }
    }

    int colors[N];
    for (int i = 0; i < n; i++) {
        int cnt[2];
        memset(cnt, 0, sizeof(cnt));
        for (int j = 0; j < ecount[i]; j++) {
            int e = enemies[i][j];
            if (e >= i) continue;
            cnt[colors[e]]++;
        }
        colors[i] = (cnt[0] > cnt[1] ? 1 : 0);
    }

    for (int i = 0; i < n; i++) {
        int cnt[2];
        memset(cnt, 0, sizeof(cnt));
        for (int j = 0; j < ecount[i]; j++) {
            int e = enemies[i][j];
            cnt[colors[e]]++;
        }
        assert(cnt[colors[i]] <= 1);
    }

    int x = colors[0];
    int cnt[2];
    memset(cnt, 0, sizeof(cnt));
    for (int i = 0; i < n; i++) cnt[colors[i]]++;
    assert(cnt[0] + cnt[1] == n);
    if (cnt[0] != cnt[1]) x = (cnt[0] > cnt[1] ? 1 : 0);
    assert(cnt[x] <= cnt[x ^ 1]);
    cout << cnt[x] << '\n';
    int k = 0;
    for (int i = 0; i < n; i++) {
        if (colors[i] == x) {
            cout << i + 1;
            cout << (k < cnt[x] - 1 ? ' ' : '\n');
            k++;
        }
    }
    return 0;
}
