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

const string YES = "It is a kind of magic";
const string NO = "There are no miracles in life";

int main() {
    long long a, b, c;
    long long x, y, z;
    cin >> a >> b >> c;
    cin >> x >> y >> z;
    long long a_need = max(x - a, 0LL);
    long long b_need = max(y - b, 0LL);
    long long a_free = max(a - x, 0LL);
    long long b_free = max(b - y, 0LL);
    if (c >= a_need + b_need && c + a_free + b_free - a_need - b_need >= z)
        cout << YES;
    else
        cout << NO;
    cout << '\n';
    return 0;
}
