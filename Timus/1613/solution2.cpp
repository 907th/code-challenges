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

const int N = 70000;

struct City {
    int id, x;
};

bool operator < (const City &a, const City &b) {
    return (a.x < b.x || (a.x == b.x && a.id < b.id));
}

int n;
City cities[N];

int main() {
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        scanf("%d", &cities[i].x);
        cities[i].id = i + 1;
    }
    sort(cities, cities + n);

    int q;
    scanf("%d", &q);
    for (int i = 0; i < q; i++) {
        int l, r, x;
        scanf("%d %d %d", &l, &r, &x);
        auto lo = lower_bound(cities, cities + n, City{l, x});
        auto up = upper_bound(cities, cities + n, City{r, x});
        printf("%d", up - lo > 0 ? 1 : 0);
    }
    printf("\n");

    return 0;
}
