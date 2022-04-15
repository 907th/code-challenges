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

int fact(int n, int k) {
    if (n == k) return k;
    else if (n < k) return n;
    else return n * fact(n - k, k);
}

int main() {
    int n;
    string s;
    cin >> n >> s;
    int k = s.length();
    cout << fact(n, k) << '\n';
    return 0;
}
