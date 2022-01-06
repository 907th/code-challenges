#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>

using namespace std;

int main() {
    int cost[2];
    int diff[2];
    cin >> cost[0];
    cin >> diff[0];
    cin >> cost[1];
    cin >> diff[1];
    int i = 0, ans = cost[0];
    while (cost[0] < cost[1]) {
        if (i == 0) {
            cost[0] += diff[0];
            ans = min(cost[0], cost[1]);
        } else {
            cost[1] -= diff[1];
            ans = max(cost[0], cost[1]);
        }
        i = i ^ 1;
    }
    cout << ans << '\n';
    return 0;
}
