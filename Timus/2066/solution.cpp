#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int a[3];
    for (int i = 0; i < 3; i++) cin >> a[i];
    sort(a, a + 3);
    cout << min(a[0] - a[1] * a[2], a[0] - a[1] - a[2]) << '\n';
    return 0;
}
