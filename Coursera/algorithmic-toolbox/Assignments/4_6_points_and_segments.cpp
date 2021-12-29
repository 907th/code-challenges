#include <iostream>
#include <algorithm>

using namespace std;

const int N = 50000 * 3 + 10;

struct Point {
    int type; // 0 - segment left end, 1 - point, 2 - segment right end
    int pos;
    int id; // point original id
};

int n, p, s;
Point a[N];
int ans[N];

bool cmp_points(const Point &a, const Point &b) {
    if (a.pos != b.pos) return a.pos < b.pos;
    return a.type < b.type;
}

int main() {
    n = 0;
    cin >> s >> p;
    for (int i = 0; i < s; i++) {
        int left, right;
        cin >> left >> right;
        a[n++] = Point{ 0, left, -1 };
        a[n++] = Point{ 2, right, -1 };
    }
    for (int i = 0; i < p; i++) {
        int value;
        cin >> value;
        a[n++] = Point{ 1, value, i };
    }

    sort(a, a + n, cmp_points);
    int k = 0;
    for (int i = 0; i < n; i++) {
        if (a[i].type == 0) {
            k++;
        } else if (a[i].type == 2) {
            k--;
        } else {
            ans[a[i].id] = k;
        }
    }

    for (int i = 0; i < p; i++) {
        cout << ans[i];
        cout << (i < p - 1 ? ' ' : '\n');
    }
}
