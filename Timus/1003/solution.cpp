#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <unordered_map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

typedef pair<int, bool> PIB;

int main() {
    while (1) {
        int n, m;
        map<int, PIB> h;
        int res = 0;
        bool consistent = true;
        cin >> n;
        if (n == -1) break;
        cin >> m;
        for (int i = 0; i < m; i++) {
            int l, r;
            string s;
            cin >> l >> r >> s;
            bool b = (s == "odd" ? true : false);
            while (consistent) {
                auto q = h.find(l);
                if (q == h.end()) {
                    h[l] = make_pair(r, b);
                    break;
                } else {
                    int qr = q->second.first;
                    bool qb = q->second.second;
                    if (qr == r) {
                        if (qb != b) consistent = false;
                        break;
                    } else {
                        l = min(qr, r) + 1;
                        r = max(qr, r);
                        b = qb ^ b;
                    }
                }
            }
            if (consistent) res++;
        }
        cout << res << '\n';
    }
    return 0;
}
