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
    int a[1000];
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) cin >> a[i];

    int s = a[0] + a[1] + a[2];
    int sum = s, pos = 1;
    for (int i = 3; i < n; i++) {
        s += a[i] - a[i - 3];
        if (s > sum) {
            sum = s;
            pos = i - 1;
        }
    }
    cout << sum << ' ' << pos + 1 << '\n';
    return 0;
}
