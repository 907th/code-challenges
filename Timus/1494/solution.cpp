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
    int n;
    cin >> n;
    stack<int> s;
    int m = 1;
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        assert(1 <= x && x <= n);
        if (m <= x) {
            while (m < x) s.push(m++);
            m++;
        } else {
            if (s.empty() || s.top() != x) {
                printf("Cheater\n");
                return 0;
            } else s.pop();
        }
    }
    assert(s.empty());
    printf("Not a proof\n");
    return 0;
}
