#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int n;
vector<int> a(n);

int count_occurances(int v, int l, int r) {
    int k = 0;
    for (int i = l; i < r; i++) {
        if (a[i] == v) k++;
    }
    return k;
}

int get_majority_element(int l, int r) {
    if (r - l == 0) return -1;
    if (r - l == 1) return a[l];

    int c = (l + r) / 2;

    // Find left majority
    int left_maj = get_majority_element(l, c);
    if (left_maj != -1) {
        int left_count = count_occurances(left_maj, l, r);
        if (left_count > (r - l) / 2) return left_maj;
    }

    // Find right majority
    int right_maj = get_majority_element(c, r);
    if (right_maj != -1) {
        int right_count = count_occurances(right_maj, l, r);
        if (right_count > (r - l) / 2) return right_maj;
    }

    return -1;
}

int main() {
    cin >> n;
    a.reserve(n);
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        a.push_back(v);
    }
    cout << (get_majority_element(0, n) != -1) << '\n';
}
