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

// Draw all vertices in graph into the same color.
// Then repeat: while there exists a vertex with 2 or 3 neighbors of same color,
// change the color of this vertex to the opposite one.
// This algo is finite because the number of edges between two sets of vertices
// will encrease after each color change.
int main() {
    const int N = 7163;
    int n;
    int k[N];
    int e[N][3];
    cin >> n;
    for (int i = 0; i < n; i++) {
        cin >> k[i];
        for (int j = 0; j < k[i]; j++) {
            cin >> e[i][j];
            e[i][j]--;
        }
    }

    int colors[N];
    memset(colors, 0, sizeof(colors));
    bool change = true;
    while (change) {
        change = false;
        for (int i = 0; i < n; i++) {
            int o = 0;
            for (int j = 0; j < k[i]; j++)
                if (colors[i] == colors[e[i][j]]) o++;
            if (o >= 2) {
                colors[i] ^= 1;
                change = true;
            }
        }
    }

    int x = colors[0];
    int cnt[2];
    memset(cnt, 0, sizeof(cnt));
    for (int i = 0; i < n; i++) cnt[colors[i]]++;
    if (cnt[0] != cnt[1]) x = (cnt[0] > cnt[1] ? 1 : 0);
    int ans[N];
    int ansk = 0;
    for (int i = 0; i < n; i++)
        if (colors[i] == x) ans[ansk++] = i + 1;
    cout << ansk << '\n';
    for (int i = 0; i < ansk; i++) {
        cout << ans[i];
        cout << (i < ansk - 1 ? ' ' : '\n');
    }
    return 0;
}
