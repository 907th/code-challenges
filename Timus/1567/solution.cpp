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
    char a[256];
    memset(a, 0, 256 * sizeof(char));
    a['a'] = 1; a['b'] = 2; a['c'] = 3;
    a['d'] = 1; a['e'] = 2; a['f'] = 3;
    a['g'] = 1; a['h'] = 2; a['i'] = 3;
    a['j'] = 1; a['k'] = 2; a['l'] = 3;
    a['m'] = 1; a['n'] = 2; a['o'] = 3;
    a['p'] = 1; a['q'] = 2; a['r'] = 3;
    a['s'] = 1; a['t'] = 2; a['u'] = 3;
    a['v'] = 1; a['w'] = 2; a['x'] = 3;
    a['y'] = 1; a['z'] = 2;
    a['.'] = 1; a[','] = 2; a['!'] = 3;
    a[' '] = 1;
    char s[1001];
    gets(s);
    int ans = 0;
    for (int i = 0; s[i]; i++) {
        if (a[s[i]] == 0) throw 1;
        ans += a[s[i]];
    }
    cout << ans << '\n';
    return 0;
}
