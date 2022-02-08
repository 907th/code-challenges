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
    const int V[6] = {10, 50, 100, 500, 1000, 5000};

    int k[6], price;
    for (int i = 0; i < 6; i++) cin >> k[i];
    cin >> price;

    int total = 0;
    int min = -1;
    for (int i = 0; i < 6; i++) {
        if (k[i] > 0 && min == -1) min = V[i];
        total += k[i] * V[i];
    }

    vector<int> ans;
    for (int i = total - min + 1; i <= total; i++)
        if (i % price == 0)
            ans.push_back(i / price);

    cout << ans.size() << '\n';
    for (int i = 0; i < ans.size(); i++) {
        cout << ans[i];
        cout << (i < ans.size() - 1 ? ' ' : '\n');
    }
    return 0;
}
