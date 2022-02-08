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
    int n;
    cin >> n;
    int left = 2;
    int right = 10;
    for (int i = 0; i < n; i++) {
        int k;
        string s;
        cin >> k >> s;
        if (s.compare("hungry") == 0) {
            if (right <= k) {
                cout << "Inconsistent\n";
                return 0;
            } else {
                left = max(left, k);
            }
        }
        if (s.compare("satisfied") == 0) {
            if (left >= k) {
                cout << "Inconsistent\n";
                return 0;
            } else {
                right = min(right, k);
            }
        }
    }
    cout << right << '\n';
    return 0;
}
