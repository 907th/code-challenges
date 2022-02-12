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
    int b, r, y;
    cin >> b >> r >> y;
    int k;
    cin >> k;
    int ans = 1;
    for (int i = 0; i < k; i++) {
        string color;
        cin >> color;
        if (color.compare("Blue") == 0) ans *= b;
        if (color.compare("Yellow") == 0) ans *= y;
        if (color.compare("Red") == 0) ans *= r;
    }
    cout << ans << '\n';
    return 0;
}
