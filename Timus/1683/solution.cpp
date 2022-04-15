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
    int n;
    cin >> n;
    vector<int> ans;
    while (n > 1) {
        ans.push_back(n / 2);
        n = (n + 1) / 2;
    }
    cout << ans.size() << '\n';
    for (int i = 0; i < ans.size(); i++) {
        cout << ans[i];
        cout << (i < ans.size() - 1 ? ' ' : '\n');
    }
    return 0;
}
