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

const double EPS = 1e-12;

int main() {
    int n;
    cin >> n;
    double sum = 0;
    bool bad = false;
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        sum += x;
        if (x == 3) bad = true;
    }
    if (bad) {
        cout << "None\n";
    } else {
        if (abs(sum / n - 5.0) < EPS) {
            cout << "Named\n";
        } else if (sum / n > 4.5 - EPS) {
            cout << "High\n";
        } else {
            cout << "Common\n";
        }
    }
    return 0;
}
