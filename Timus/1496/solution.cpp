#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>

using namespace std;

int main() {
    int n;
    cin >> n;
    map<string, int> h;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        auto p = h.find(s);
        if (p == h.end()) {
            h[s] = 1;
        } else {
            h[s] = h[s] + 1;
        }
    }
    for (auto i : h) {
        if (i.second > 1) cout << i.first << '\n';
    }
    return 0;
}
