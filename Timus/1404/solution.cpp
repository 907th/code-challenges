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
#include <cassert>

using namespace std;

char n2a(int n) {
    return 'a' + n;
}

int a2n(char a) {
    return a - 'a';
}

int main() {
    const int N = 101;

    int n;
    char s[N];
    gets(s);
    n = strlen(s);

    char ans[N];
    int ans_len = 0;
    int add = 5;
    for (int i = 0; i < n; i++) {
        int num = a2n(s[i]);
        ans[ans_len++] = n2a((num + 26 - add) % 26);
        add = num;
    }

    ans[ans_len++] = 0;
    cout << ans << '\n';
    return 0;
}
