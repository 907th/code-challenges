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

int main() {
    int x, y;
    cin >> x >> y;
    if (x > 0 && y > 0 && (x + y) & 1) {
        cout << y << ' ' << x << '\n';
    } else {
        cout << x << ' ' << y << '\n';
    }
    return 0;
}
