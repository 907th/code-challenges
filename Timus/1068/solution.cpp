#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int n;
    cin >> n;
    int s = abs(n) * (abs(n) + 1) / 2;
    cout << (n <= 0 ? -s + 1 : s) << '\n';
    return 0;
}
