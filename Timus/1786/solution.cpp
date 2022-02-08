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

const char *S = "Sandro";

int cost(char a, char b) {
    if (a == b) return 0;
    if (tolower(a) == tolower(b)) return 1;
    if (isupper(a) == isupper(b)) return 1;
    return 2;
}

int solve(const char *s) {
    int res = 0;
    for (int i = 0; i < 6; i++) res += cost(s[i], S[i]);
    return res;
}

int main() {
    char s[201];
    gets(s);
    int len = strlen(s);
    int ans = 1 << 30;
    for (int i = 0; i < len - 5; i++) ans = min(ans, solve(s + i));
    cout << (ans * 5) << '\n';
    return 0;
}
