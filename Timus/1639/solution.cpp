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
    int n, m;
    cin >> n >> m;
    if (n * m % 2 == 1)
        cout << "[second]=:]\n";
    else
        cout << "[:=[first]\n";
    return 0;
}
