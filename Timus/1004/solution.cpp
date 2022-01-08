#include <iostream>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

const int N = 100;
const int INF = 1 << 29;

typedef vector<int> VI;

string print_vec(const VI &v) {
    string s;
    for (int i = 0; i < v.size(); i++) {
        if (i > 0) s = s + " ";
        s = s + to_string(i);
    }
    return s;
}

// Use modification of Floyd-Warshall algo to find all cycles of form ... -> a -> k -> b -> ...
// where path from a to b is the shortest one including only vertices 1 .. k - 1.
// See https://www.quora.com/Can-Floyd-Warshall-algorithm-be-used-to-find-shortest-cycle-in-an-undirected-graph/answer/Jay-Chu-28
int main() {
    while (1) {
        int n, m;
        int roads[N][N];
        for (int a = 0; a < N; a++)
            for (int b = 0; b < N; b++)
                roads[a][b] = INF;
        cin >> n;
        if (n == -1) break;
        cin >> m;
        for (int i = 0; i < m; i++) {
            int a, b, l;
            cin >> a >> b >> l;
            a--;
            b--;
            roads[a][b] = min(roads[a][b], l);
            roads[b][a] = min(roads[b][a], l);
        }
        int res_len = INF;
        VI res_path;
        int dist[N][N];
        VI path[N][N];
        for (int a = 0; a < n; a++)
            for (int b = 0; b < n; b++) {
                dist[a][b] = roads[a][b];
                path[a][b].push_back(a);
                path[a][b].push_back(b);
            }
        for (int k = 0; k < n; k++) {
            for (int a = 0; a < n; a++)
                for (int b = 0; b < n; b++) {
                    if (a == b || a == k || b == k) continue;
                    if (res_len > dist[a][b] + roads[a][k] + roads[b][k]) {
                        res_len = dist[a][b] + roads[a][k] + roads[b][k];
                        res_path = path[a][b];
                        res_path.push_back(k);
                        // printf("New answer %s (length %d)\n", print_vec(res_path).c_str(), res_len);
                    }
                }
            for (int a = 0; a < n; a++)
                for (int b = 0; b < n; b++) {
                    if (a == b || a == k || b == k) continue;
                    if (dist[a][b] > dist[a][k] + dist[k][b]) {
                        dist[a][b] = dist[a][k] + dist[k][b];
                        VI new_path = path[a][k];
                        new_path.pop_back();
                        new_path.insert(new_path.end(), path[k][b].begin(), path[k][b].end());
                        path[a][b] = new_path;
                        // printf("New path %s (length %d)\n", print_vec(path[a][b]).c_str(), dist[a][b]);
                    }
                }
        }
        if (res_len == INF) {
            cout << "No solution.\n";
        } else {
            for (int i = 0; i < res_path.size(); i++) {
                cout << (res_path[i] + 1);
                cout << (i < res_path.size() - 1 ? ' ' : '\n');
            }
        }
    }
    return 0;
}
