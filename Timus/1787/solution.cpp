#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int k, n;
    cin >> k >> n;
    int s = 0;
    for (int i = 0; i < n; i++) {
        int a;
        cin >> a;
        s += a;
        s -= k;
        s = max(s, 0);
    }
    cout << s << '\n';
    return 0;
}
