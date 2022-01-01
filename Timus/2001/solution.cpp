#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int a[3], b[3];
    for (int i = 0; i < 3; i++) {
        cin >> a[i] >> b[i];
    }
    int aa, bb;
    aa = a[0] - a[2];
    bb = b[0] - b[1];
    cout << aa << ' ' << bb << '\n';
    return 0;
}
