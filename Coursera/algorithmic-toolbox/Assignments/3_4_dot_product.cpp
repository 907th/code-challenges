#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

long long max_dot_product(vector<int> a, vector<int> b) {
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    long long result = 0;
    for (int i = 0; i < a.size(); i++)
        result += (long long) a[i] * b[i];
    return result;
}

int main() {
    int n;
    cin >> n;
    vector<int> a(n), b(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    for (int i = 0; i < n; i++) cin >> b[i];
    cout << max_dot_product(a, b) << endl;
}
