#include <iostream>
#include <iomanip>
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

bool hamming_ok(const int x[7]) {
    return (
        x[4] == (x[1] + x[2] + x[3]) % 2 &&
        x[5] == (x[0] + x[2] + x[3]) % 2 &&
        x[6] == (x[0] + x[1] + x[3]) % 2
    );
}

int main() {
    int x[7];
    for (int i = 0; i < 7; i++) cin >> x[i];

    if (!hamming_ok(x)) {
        for (int i = 0; i < 7; i++) {
            x[i] ^= 1;
            if (hamming_ok(x)) break;
            x[i] ^= 1;
        }
    }

    for (int i = 0; i < 7; i++) {
        cout << x[i];
        cout << (i < 6 ? ' ' : '\n');
    }
    return 0;
}
