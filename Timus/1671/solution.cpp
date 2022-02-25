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

// The following two functions implement disjoint set data structure

int get_color(vector<int> &colors, int i) {
    if (colors[i] == i) return i;
    colors[i] = get_color(colors, colors[i]);
    return colors[i];
}

bool merge_colors(vector<int> &colors, int i, int j) {
    int ci = get_color(colors, i);
    int cj = get_color(colors, j);
    if (ci == cj) return false;
    if (rand() & 1) swap(ci, cj);
    colors[ci] = cj;
    return true;
}

int main() {
    int n, m, q;
    vector<PII> edges;
    vector<int> tears;
    vector<bool> usage;
    cin >> n >> m;
    for (int i = 0; i < m; i++) {
        PII e;
        cin >> e.first >> e.second;
        e.first--;
        e.second--;
        edges.push_back(e);
        usage.push_back(true);
    }
    cin >> q;
    for (int i = 0; i < q; i++) {
        int e;
        cin >> e;
        e--;
        tears.push_back(e);
        usage[e] = false;
    }

    vector<vector<int>> graph;
    vector<int> colors;
    int different_colors = 0;
    for (int i = 0; i < n; i++) {
        graph.push_back(vector<int>{});
        colors.push_back(-1);
    }
    for (int i = 0; i < m; i++)
        if (usage[i]) {
            PII edge = edges[i];
            graph[edge.first].push_back(edge.second);
            graph[edge.second].push_back(edge.first);
        }
    for (int i = 0; i < n; i++) {
        if (colors[i] != -1) continue;
        queue<int> q;
        q.push(i);
        colors[i] = i;
        while (!q.empty()) {
            int v = q.front();
            q.pop();
            for (int u : graph[v])
                if (colors[u] == -1) {
                    q.push(u);
                    colors[u] = i;
                }
        }
        different_colors++;
    }

    vector<int> ans;
    reverse(tears.begin(), tears.end());
    for (int e : tears) {
        ans.push_back(different_colors);
        int i = edges[e].first;
        int j = edges[e].second;
        if (merge_colors(colors, i, j)) different_colors--;
    }

    reverse(ans.begin(), ans.end());
    for (int i = 0; i < ans.size(); i++) {
        cout << ans[i];
        cout << (i < ans.size() - 1 ? ' ' : '\n');
    }

    return 0;
}
