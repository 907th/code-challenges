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
#include <cctype>
#include <cassert>

using namespace std;

// Hint: Consider one heap with N nuts. The numbers of steps to divide it
// into N heaps of 1 nut in each is always equal to (N - 1) / 2. So the total number
// of steps in this game is always the same and it doesn't depend on the actual move.
int main() {
    int n;
    cin >> n;
    int steps = 0;
    for (int i = 0; i < n; i++) {
        int p;
        cin >> p;
        steps += (p - 1) / 2;
    }
    cout << (steps & 1 ? "Daenerys" : "Stannis") << '\n';
    return 0;
}
