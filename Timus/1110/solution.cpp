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

int main() {
    int n, m, y;
    cin >> n >> m >> y;
    vector<int> res;
    for (int i = 1; i < m; i++) {
        int z = 1;
        for (int j = 0; j < n; j++) z = (z * i) % m;
        if (z == y) res.push_back(i);
    }
    if (res.size() == 0) {
        cout << "-1\n";
    } else {
        for (int i = 0; i < res.size(); i++) {
            cout << res[i];
            cout << (i < res.size() - 1 ? ' ' : '\n');
        }
    }
    return 0;
}
