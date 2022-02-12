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
    string s;
    long long n;
    cin >> s >> n;
    long long ans = 0;
    const long long P = 256;
    for (int i = 0; i < 4; i++) {
        long long r = n % P;
        ans = ans * P + r;
        n /= P;
    }
    cout << ans << '\n';
    return 0;
}
