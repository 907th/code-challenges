#include <iostream>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    int t1, t2;
    int s = 0;
    cin >> t1 >> t2;
    for (int i = 0; i < 10; i++) {
        int v;
        cin >> v;
        s += v;
    }
    if (t2 - 20 * s < t1) {
        cout << "Dirty debug :(\n";
    } else {
        cout << "No chance.\n";
    }
    return 0;
}
