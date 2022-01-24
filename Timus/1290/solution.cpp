#include <iostream>
#include <iomanip>
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

vector<int> perform_algo(vector<int> &v) {
    vector<int> res;
    res.push_back(v.size());
    sort(v.begin(), v.end());
    int i = 1, j = 0, n = v.size();
    while (j < v.size()) {
        int x = v[j], k = 1;
        while (j + 1 < v.size() && v[j + 1] == x) {
            j++;
            k++;
        }
        while (i < x) {
            res.push_back(n);
            i++;
        }
        n -= k;
        if (n > 0) {
            res.push_back(n);
            i++;
        }
        j++;
    }
    return res;
}

int main() {
    int n;
    cin >> n;

    if (n == 0) {
        cout << "\n";
        return 0;
    }

    vector<int> v;
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        v.push_back(x);
    }

    v = perform_algo(v);
    v = perform_algo(v);
    for (int x : v) {
        cout << x << '\n';
    }

    return 0;
}
