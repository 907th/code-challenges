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
    int h, w, n;
    cin >> h >> w >> n;
    int hh = 1, ww = 0, pp = 0;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        if (ww + s.size() + pp > w) {
            hh++;
            ww = s.size();
            pp = 1;
        } else {
            ww += s.size() + pp;
            pp = 1;
        }
    }
    int res = (hh + h - 1) / h;
    cout << res << '\n';
    return 0;
}
