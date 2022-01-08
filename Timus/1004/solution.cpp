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
// where path from a to b is the shortest one consisting of vertices 1 .. k - 1 only.
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
        int prev[N][N];
        for (int a = 0; a < n; a++)
            for (int b = 0; b < n; b++) {
                dist[a][b] = roads[a][b];
                prev[a][b] = (roads[a][b] == INF ? -1 : a);
            }
        for (int k = 0; k < n; k++) {
            // Find shortest cycle
            int res_a, res_b, res_k;
            res_a = -1;
            for (int a = 0; a < n; a++)
                for (int b = 0; b < n; b++) {
                    if (a == b || a == k || b == k) continue;
                    if (res_len > dist[a][b] + roads[a][k] + roads[b][k]) {
                        res_len = dist[a][b] + roads[a][k] + roads[b][k];
                        res_a = a; res_b = b; res_k = k;
                        // printf("New cycle found %d -> %d via %d (length %d)\n", a, b, k, res_len);
                    }
                }
            // Reconstruct vertices of the cycle
            if (res_a != -1) {
                res_path.clear();
                while (res_b != res_a) {
                    res_path.push_back(res_b);
                    res_b = prev[res_a][res_b];
                }
                res_path.push_back(res_a);
                res_path.push_back(res_k);
                // printf("New answer %s (length %d)\n", print_vec(res_path).c_str(), res_len);
            }
            // Relax distances (Floyd-Warshall algo)
            for (int a = 0; a < n; a++)
                for (int b = 0; b < n; b++) {
                    if (a == b || a == k || b == k) continue;
                    if (dist[a][b] > dist[a][k] + dist[k][b]) {
                        dist[a][b] = dist[a][k] + dist[k][b];
                        prev[a][b] = prev[k][b];
                        // printf("New path %d -> %d via %d (length %d)\n", a, b, k, dist[a][b]);
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
