#include <iostream>
#include <cassert>
#include <vector>

using namespace std;

int binary_search(const vector<int> &a, int x) {
    int l = 0, r = a.size();
    while (l < r) {
        int c = (l + r) / 2;
        if (a[c] < x) l = c + 1;
        else r = c;
    }
    return l < a.size() && a[l] == x ? l : -1;
}

int linear_search(const vector<int> &a, int x) {
    for (int i = 0; i < a.size(); ++i) {
        if (a[i] == x) return i;
    }
    return -1;
}

int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < a.size(); i++) cin >> a[i];

    int m;
    cin >> m;
    vector<int> b(m);
    for (int i = 0; i < m; ++i) cin >> b[i];

    for (int i = 0; i < m; ++i) {
        //cout << linear_search(a, b[i]);
        cout << binary_search(a, b[i]);
        cout << (i < m - 1 ? ' ' : '\n');
    }
}
