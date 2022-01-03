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
    int s = n * (n + 1) / 2;
    cout << (s % 2 == 1 ? "grimy" : "black") << '\n';
    return 0;
}
