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

int sum(int n) {
    int s = 0;
    while (n > 0) {
        s += n % 10;
        n /= 10;
    }
    return s;
}

bool lucky(int n) {
    return sum(n / 1000) == sum(n % 1000);
}

bool solve(int n) {
    return lucky(n + 1) || lucky(n - 1);
}

int main() {
    int n;
    cin >> n;
    cout << (solve(n) ? "Yes" : "No") << '\n';
    return 0;
}
