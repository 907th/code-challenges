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

struct N {
    int pos;
    int id;
    int acc;
};

bool cmp_func(N &a, N &b) {
    if (a.acc != b.acc) return a.acc > b.acc;
    return a.pos < b.pos;
}

int main() {
    int n;
    N t[150000];
    cin >> n;
    for (int i = 0; i < n; i++) {
        t[i].pos = i;
        cin >> t[i].id >> t[i].acc;
    }
    sort(t, t + n, cmp_func);
    for (int i = 0; i < n; i++)
        cout << t[i].id << ' ' << t[i].acc << '\n';
    return 0;
}
