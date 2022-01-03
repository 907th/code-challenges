#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    map<int, int> m;
    for (int p = 0; p < 3; p++) {
        int n;
        cin >> n;
        for (int i = 0; i < n; i++) {
            int v;
            cin >> v;
            auto x = m.find(v);
            if (x != m.end())
                m[v] = x->second + 1;
            else
                m[v] = 1;
        }
    }
    int res = 0;
    for (auto i : m)
        if (i.second == 3) res++;
    cout << res << '\n';
    return 0;
}
