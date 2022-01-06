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

int main() {
    int a, b;
    cin >> a >> b;
    int s = (b + 1) / 2;
    if (a > 1) s -= a / 2;
    cout << s << '\n';
    return 0;
}
