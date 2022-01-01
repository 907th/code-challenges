#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    string a, b;
    cin >> a >> b;
    bool ans = false;
    ans = ans || (stoi(a) % 2 == 0);
    ans = ans || (stoi(b) % 2 == 1);
    cout << (ans ? "yes" : "no") << '\n';
    return 0;
}
