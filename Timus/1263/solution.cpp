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
    int n, m;
    int a[10000];
    memset(a, 0, sizeof(a));
    cin >> n >> m;
    for (int i = 0; i < m; i++) {
        int x;
        cin >> x;
        a[x - 1]++;
    }
    cout.setf(ios::fixed);
    cout.precision(2);
    for (int i = 0; i < n; i++)
        cout << 100.0 * (double) a[i] / m << "%\n";
    return 0;
}
