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
    int n, k;
    cin >> n >> k;
    cout << max(max(k - 1, n - k) - 2, 0) << '\n';
    return 0;
}
