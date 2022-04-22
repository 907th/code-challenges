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
typedef vector<int> VI;

const int N = 70000;

struct Q {
    int l, r, x;
};

int n, q;
int numbers[N];
Q queries[N];

VI q_start[N];
VI q_finish[N];
set<PII> q_open;
int ans[N];

// The solution:
// - Move along the original array of numbers from left to right.
// - On each step maintain a set of all open queries.
// - For each next number check if there exist an open query with such a number.
//
// Cons: This solution is not on-line (it needs to know all queries beforehand).
// There exist a simpler and on-line solution (see solution2.cpp).
int main() {
    scanf("%d", &n);
    for (int i = 0; i < n; i++) scanf("%d", &numbers[i]);
    scanf("%d", &q);
    for (int i = 0; i < q; i++) {
        Q query;
        scanf("%d %d %d", &query.l, &query.r, &query.x);
        query.l--;
        query.r--;
        queries[i] = query;
        q_start[query.l].push_back(i);
        q_finish[query.r].push_back(i);
    }

    memset(ans, 0, sizeof(ans));
    for (int i = 0; i < n; i++) {
        // Open new queries
        for (int j = 0; j < q_start[i].size(); j++) {
            int q_id = q_start[i][j];
            Q query = queries[q_id];
            q_open.insert(make_pair(query.x, q_id));
        }

        // Process open queries
        while (1) {
            auto it = q_open.lower_bound(make_pair(numbers[i], -1));
            if (it != q_open.end() && it->first == numbers[i]) {
                ans[it->second] = 1;
                q_open.erase(it);
            } else break;
        }

        // Close finished queries
        for (int j = 0; j < q_finish[i].size(); j++) {
            int q_id = q_finish[i][j];
            Q query = queries[q_id];
            int removed_count = q_open.erase(make_pair(query.x, q_id));
            assert(ans[q_id] ^ removed_count);
        }
    }
    assert(q_open.size() == 0);

    for (int i = 0; i < q; i++) printf("%d", ans[i]);
    printf("\n");
    return 0;
}
