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

int n, x, y;

bool valid(int x, int y) {
    return (1 <= x && x <= n && 1 <= y && y <= n);
}

int king() {
    int ans = 0;
    for (int dx = -1; dx <= 1; dx++)
        for (int dy = -1; dy <= 1; dy++)
            if (dx != 0 || dy != 0)
                if (valid(x + dx, y + dy))
                    ans++;
    return ans;
}

int knight() {
    int ans = 0;
    for (int dx = -2; dx <= 2; dx++)
        for (int dy = -2; dy <= 2; dy++)
            if (dx * dy == 2 || dx * dy == -2)
                if (valid(x + dx, y + dy))
                    ans++;
    return ans;
}

int bishop() {
    int ans = 0;
    ans += max(0, min(x - 1, y - 1));
    ans += max(0, min(n - x, n - y));
    ans += max(0, min(x - 1, n - y));
    ans += max(0, min(n - x, y - 1));
    return ans;
}

int rook() {
    int ans = 0;
    ans += max(0, x - 1);
    ans += max(0, n - x);
    ans += max(0, y - 1);
    ans += max(0, n - y);
    return ans;
}

int queen() {
    int ans = 0;
    ans += bishop();
    ans += rook();
    return ans;
}

int main() {
    cin >> n >> x >> y;
    cout << "King: "   << king()   << '\n';
    cout << "Knight: " << knight() << '\n';
    cout << "Bishop: " << bishop() << '\n';
    cout << "Rook: "   << rook()   << '\n';
    cout << "Queen: "  << queen()  << '\n';
    return 0;
}
