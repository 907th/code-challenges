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

int main() {
    char buf[256];
    int blen = 0;
    while (1) {
        char c = getc(stdin);
        if (isalpha(c)) buf[blen++] = c;
        else {
            if (blen > 0) {
                reverse(buf, buf + blen);
                buf[blen++] = 0;
                fprintf(stdout, "%s", buf);
            }
            blen = 0;
            if (c == EOF) break;
            else putc(c, stdout);
        }
    }
    return 0;
}
