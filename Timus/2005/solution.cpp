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

int main() {
    int d[5][5];
    for (int i = 0; i < 5; i++)
        for (int j = 0; j < 5; j++) cin >> d[i][j];
    int p[5];
    for (int i = 0; i < 5; i++) p[i] = i;
    int ans = 1 << 30;
    int ans_p[5];
    while (1) {
        if (p[3] != 2) {
            int cur = 0;
            for (int i = 1; i < 5; i++) cur += d[p[i - 1]][p[i]];
            if (cur < ans) {
                ans = cur;
                memcpy(ans_p, p, sizeof(p));
            }
        }
        if (!next_permutation(p + 1, p + 4)) break;
    }
    cout << ans << '\n';
    for (int i = 0; i < 5; i++) {
        cout << (ans_p[i] + 1);
        cout << (i < 4 ? ' ' : '\n');
    }
    return 0;
}
