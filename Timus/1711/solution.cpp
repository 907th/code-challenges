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

typedef pair<int, int> PII;

PII MP(int a, int b) {
    return make_pair(a, b);
}

int main() {
    int n;
    string names[16][3];
    cin >> n;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < 3; j++)
            cin >> names[i][j];
    int order[16];
    for (int i = 0; i < n; i++) {
        cin >> order[i];
        order[i]--;
    }

    // Reorder
    string names2[16][3];
    for (int i = 0; i < n; i++)
        for (int j = 0; j < 3; j++)
            names2[i][j] = names[order[i]][j];

    // BFS
    int prev[16][3];
    queue<PII> q;
    memset(prev, -1, sizeof(prev));
    for (int j = 0; j < 3; j++) {
        prev[0][j] = j;
        q.push(MP(0, j));
    }
    while (!q.empty()) {
        int i = q.front().first;
        int j = q.front().second;
        q.pop();
        if (i == n - 1) continue;
        string v = names2[i][j];
        for (int k = 0; k < 3; k++) {
            if (prev[i + 1][k] != -1) continue;
            string u = names2[i + 1][k];
            if (u.compare(v) < 0) continue;
            prev[i + 1][k] = j;
            q.push(MP(i + 1, k));
        }
    }

    // Get answer
    int k = -1;
    for (int j = 0; j < 3; j++)
        if (prev[n - 1][j] != -1) {
            k = j;
            break;
        }
    if (k == -1) {
        cout << "IMPOSSIBLE\n";
        return 0;
    }
    string ans[16];
    for (int i = n - 1; i >= 0; i--) {
        ans[i] = names2[i][k];
        k = prev[i][k];
    }
    for (int i = 0; i < n; i++) cout << ans[i] << '\n';
    return 0;
}
