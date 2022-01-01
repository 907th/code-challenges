#include <iostream>
#include <algorithm>
#include <vector>
#include <cstdio>
#include <cmath>

using namespace std;

string solve(int n) {
    if (n >= 1 && n <= 4) return "few";
    if (n >= 5 && n <= 9) return "several";
    if (n >= 10 && n <= 19) return "pack";
    if (n >= 20 && n <= 49) return "lots";
    if (n >= 50 && n <= 99) return "horde";
    if (n >= 100 && n <= 249) return "throng";
    if (n >= 250 && n <= 499) return "swarm";
    if (n >= 500 && n <= 999) return "zounds";
    if (n >= 1000) return "legion";
    throw 1;
}

int main() {
    int n;
    cin >> n;
    cout << solve(n) << '\n';
    return 0;
}
