#include <iostream>
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
    const int N = 100;
    int n;
    vector<int> edges[N];
    cin >> n;
    for (int i = 0; i < n; i++) {
        while (1) {
            int v;
            cin >> v;
            if (v == 0) break;
            edges[i].push_back(v - 1);
        }
    }

    int visits[N];
    memset(visits, -1, sizeof(visits));
    for (int i = 0; i < n; i++) {
        if (visits[i] != -1) continue;
        stack<int> st;
        visits[i] = 0;
        st.push(i);
        while (!st.empty()) {
            int v = st.top();
            st.pop();
            for (int u : edges[v])
                if (visits[u] == -1) {
                    visits[u] = visits[v] ^ 1;
                    st.push(u);
                }
        }
    }

    vector<int> ans;
    for (int i = 0; i < n; i++)
        if (visits[i] == 0)
            ans.push_back(i);

    cout << ans.size() << '\n';;
    for (int i = 0 ; i < ans.size(); i++) {
        cout << (ans[i] + 1);
        cout << (i < ans.size() - 1 ? ' ' : '\n');
    }

    return 0;
}
