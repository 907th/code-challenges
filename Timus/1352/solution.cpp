#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

// http://oeis.org/A000043
const int MERSENNE_EXPONENTS[] = {
    2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107,
    127, 521, 607, 1279, 2203, 2281, 3217, 4253,
    4423, 9689, 9941, 11213, 19937, 21701, 23209,
    44497, 86243, 110503, 132049, 216091, 756839,
    859433, 1257787, 1398269, 2976221, 3021377,
    6972593, 13466917, 20996011, 24036583, 25964951,
    30402457, 32582657, 37156667, 42643801, 43112609,
    57885161
};

// Almost all powers excpet for the smallest ones can be found in
// the task description and example. Or you can just google them.
int main() {
    int t;
    cin >> t;
    for (int i = 0; i < t; i++) {
        int p;
        cin >> p;
        cout << MERSENNE_EXPONENTS[p - 1] << '\n';
    }
    return 0;
}
