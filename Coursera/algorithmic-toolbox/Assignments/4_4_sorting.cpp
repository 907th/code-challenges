#include <iostream>
#include <vector>
#include <cstdlib>

using namespace std;

typedef pair<int, int> PII;

PII partition3(vector<int> &a, int l, int r) {
    int x = a[l];
    int m1 = l;
    int m2 = l;
    for (int i = l + 1; i <= r; i++) {
        if (a[i] < x) {
            swap(a[i], a[m1]);
            m1++;
            m2++;
            swap(a[i], a[m2]);
        } else if (a[i] == x) {
            m2++;
            swap(a[i], a[m2]);
        }
    }
    return make_pair(m1, m2);
}

void randomized_quick_sort(vector<int> &a, int l, int r) {
    if (l >= r) return;
    int k = l + rand() % (r - l + 1);
    swap(a[l], a[k]);
    PII m = partition3(a, l, r);
    randomized_quick_sort(a, l, m.first - 1);
    randomized_quick_sort(a, m.second + 1, r);
}

int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < a.size(); ++i) cin >> a[i];
    randomized_quick_sort(a, 0, a.size() - 1);
    for (int i = 0; i < a.size(); ++i) {
        cout << a[i];
        cout << (i < a.size() - 1 ? ' ' : '\n');
    }
}
