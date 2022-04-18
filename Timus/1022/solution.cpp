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
    const int N = 100;
    int n;
    vector<int> children[N + 1];

    cin >> n;
    for (int i = 1; i <= n; i++) {
        while (1) {
            int x;
            cin >> x;
            if (x == 0) break;
            children[i].push_back(x);
        }
    }

    int parents[N + 1];
    memset(parents, 0, sizeof(parents));
    for (int v = 1; v <= n; v++)
        for (int u : children[v])
            parents[u]++;
    for (int v = 1; v <= n; v++)
        if (parents[v] == 0) {
            children[0].push_back(v);
            parents[v]++;
        }

    vector<int> answer;
    queue<int> q;
    q.push(0);
    while (!q.empty()) {
        int v = q.front();
        answer.push_back(v);
        q.pop();
        for (int u : children[v]) {
            parents[u]--;
            if (parents[u] == 0) q.push(u);
        }
    }

    assert(answer.size() == n + 1);
    for (int i = 1; i < answer.size(); i++) {
        cout << answer[i];
        cout << (i < answer.size() - 1 ? ' ' : '\n');
    }

    return 0;
}
