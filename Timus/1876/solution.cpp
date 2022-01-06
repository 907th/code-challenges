#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    cout << max(2 * a + 39, 2 * b + 40) << '\n';
    return 0;
}
