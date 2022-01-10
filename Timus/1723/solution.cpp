#include <iostream>
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

typedef pair<int, int> PII;

int main() {
    int n;
    char s[50];
    gets(s);
    n = strlen(s);

    const int B1 = 17;
    const int B2 = 57;
    int p1[50], p2[50];
    int h1[50], h2[50];
    p1[0] = 1;
    p2[0] = 1;
    h1[0] = s[0];
    h2[0] = s[0];
    for (int i = 1; i < n; i++) {
        p1[i] = p1[i - 1] * B1;
        p2[i] = p2[i - 1] * B2;
        h1[i] = h1[i - 1] * B1 + s[i];
        h2[i] = h2[i - 1] * B2 + s[i];
    }

    int res_count = 0;
    string res_str;
    for (int i = 0; i < n; i++)
        for (int j = i; j < n; j++) {
            PII x = make_pair(
              h1[j] - (i > 0 ? h1[i - 1] * p1[j - i + 1] : 0),
              h2[j] - (i > 0 ? h2[i - 1] * p2[j - i + 1] : 0)
            );
            int count = 0;
            string str(s + i, s + j + 1);
            for (int k = 0; k < n - j; k++) {
                PII xk = make_pair(
                  h1[j + k] - (i + k > 0 ? h1[i + k - 1] * p1[j - i + 1] : 0),
                  h2[j + k] - (i + k > 0 ? h2[i + k - 1] * p2[j - i + 1] : 0)
                );
                if (x == xk) count++;
            }
            // cout << "Substr " << str << ' ' << count << '\n';
            if (count >= res_count) {
                res_count = count;
                res_str = str;
            }
        }
    assert(res_count > 0);
    cout << res_str << '\n';
    return 0;
}
