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

void check_for_small_values() {
    const int N = 35;
    int w[N], k[N];
    w[0] = 2;
    w[1] = 1;
    k[1] = 1;
    for (int i = 2; i < N; i++) {
        bool ok = false;
        for (int j = 1; j <= i; j <<= 1) {
            if (w[i - j] == 2) {
                w[i] = 1;
                k[i] = j;
                ok = true;
                break;
            }
        }
        if (!ok) {
            w[i] = 2;
        }
    }
    for (int i = 1; i < N; i++) {
        cout << i << ": " << w[i];
        if (w[i] == 1) cout << ' ' << k[i];
        cout << '\n';
    }
}

int main() {
    //check_for_small_values();

    char num[251];
    gets(num);

    // Get reminder of divide by 3
    int rem = 0;
    for (int i = 0; num[i]; i++) {
        rem = rem * 10 + (num[i] - '0');
        rem = rem % 3;
    }

    switch (rem) {
        case 0: cout << "2\n"; break;
        case 1: cout << "1\n1\n"; break;
        case 2: cout << "1\n2\n"; break;
    }

    return 0;
}
