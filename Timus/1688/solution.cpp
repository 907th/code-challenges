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
    long long n;
    int m;
    cin >> n;
    cin >> m;
    n *= 3;
    long long sum = 0;
    for (int i = 0; i < m; i++) {
        long long q;
        cin >> q;
        sum += q;
        if (sum > n) {
            cout << "Free after " << (i + 1) << " times.\n";
            return 0;
        }
    }
    cout << "Team.GOV!\n";
    return 0;
}
