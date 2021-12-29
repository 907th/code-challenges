#include <iostream>
#include <vector>

using namespace std;

const int N = 100000 + 5;

typedef long long LL;

int n;
int a[N];
int b[N];

LL merge(int l, int c, int r) {
    LL ans = 0;
    LL cnt = 0; // Number of elements from right part which are already merged
    int i = l;
    int p1 = l;
    int p2 = c;
    while (1) {
        if (p1 >= c) break;
        if (p2 >= r) break;
        if (a[p1] <= a[p2]) {
            b[i++] = a[p1++];
            ans += cnt;
        } else {
            b[i++] = a[p2++];
            cnt++;
        }
    }
    while (p1 < c) {
        b[i++] = a[p1++];
        ans += cnt;
    }
    while (p2 < r) {
        b[i++] = a[p2++];
    }
    for (int j = l; j < r; j++) a[j] = b[j];
    return ans;
}

LL get_number_of_inversions(int l, int r) {
    if (r - l <= 1) return 0;
    LL ans = 0;
    int c = (l + r) / 2;
    ans += get_number_of_inversions(l, c);
    ans += get_number_of_inversions(c, r);
    ans += merge(l, c, r);
    return ans;
}

int main() {
    cin >> n;
    for (int i = 0; i < n; i++) cin >> a[i];
    cout << get_number_of_inversions(0, n) << '\n';
}
