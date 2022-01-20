#include <iostream>
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

int main() {
    int x, y, c;
    cin >> x >> y >> c;
    if (x + y < c) {
        cout << "Impossible\n";
    } else {
        int a = min(x, c);
        int b = c - a;
        cout << a << ' ' << b << '\n';
    }
    return 0;
}
