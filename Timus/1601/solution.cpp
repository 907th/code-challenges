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
    const int N = 10001;
    char s[N];
    bool first = true;
    while (gets(s)) {
        for (int i = 0; s[i]; i++) {
            char c = s[i];
            if (isalpha(c)) {
                printf("%c", first ? c : tolower(c));
                first = false;
            } else {
                printf("%c", c);
                if (c == '.' || c == '!' || c == '?') first = true;
            }
        }
        printf("\n");
    }
    return 0;
}
