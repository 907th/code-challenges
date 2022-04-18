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
    int l, k, h;
    cin >> l >> k >> h;
    int a = l / k * h;
    int b = (l + k - 1) / k * h;
    cout << fixed << setprecision(6);
    cout << (double) a << ' ' << (double) b << '\n';
    return 0;
}
