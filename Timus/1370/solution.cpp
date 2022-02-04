#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    m = m % n;
    char first[10];
    int ans_len = 0;
    char ans[11];
    for (int i = 0; i < n; i++) {
        int d;
        cin >> d;
        if (i < 10) first[i] = '0' + d;
        if (m == 0) {
            if (ans_len < 10) ans[ans_len++] = '0' + d;
        } else m--;
    }
    for (int i = 0; ans_len < 10; i++) ans[ans_len++] = first[i];
    ans[10] = 0;
    cout << ans << '\n';
    return 0;
}
