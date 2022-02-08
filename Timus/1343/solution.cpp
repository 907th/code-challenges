#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

bool prime(long long num) {
    if (num < 2) return false;
    for (long long i = 2; i * i <= num; i++)
        if (num % i == 0) return false;
    return true;
}

int main() {
    long long n, num;
    string s;
    cin >> n;
    if (n > 0) {
        cin >> s;
        num = stoll(s);
    } else {
        num = 0;
    }
    for (int i = 0; i < 12 - n; i++) num = num * 10;
    while (true) {
        if (prime(num)) {
            cout << setw(12) << setfill('0') << num << '\n';
            break;
        }
        num++;
    }
    return 0;
}
