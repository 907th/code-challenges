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

const int N = 1000000;

const int A = 40000;
const int B = 2000;
const int C = 100;

const int P = 26;

int a[26];
int b[26][26];
int c[26][26][26];

bool push(int x1, int x2, int x3) {
    if (++a[x3] > A) return false;
    if (++b[x2][x3] > B) return false;
    if (++c[x1][x2][x3] > C) return false;
    return true;
}

int next(int x1, int x2, int x3, int p1, int p2, int p3) {
    int x4 = (x1 * p1 + x2) * p2 + x3;
    x4 = (x4 % p3) % P;
    assert(0 <= x4 && x4 < P);
    return x4;
}

int check(int p1, int p2, int p3) {
    memset(a, 0, sizeof(a));
    memset(b, 0, sizeof(b));
    memset(c, 0, sizeof(c));
    int x1 = 0, x2 = 1, x3 = 2;
    a[x1] = 1;
    a[x2] = 1;
    a[x3] = 1;
    b[x1][x2] = 1;
    b[x2][x3] = 1;
    c[x1][x2][x3] = 1;
    for (int i = 3; i < N; i++) {
        int x4 = next(x1, x2, x3, p1, p2, p3);
        if (!push(x2, x3, x4)) return i;
        x1 = x2;
        x2 = x3;
        x3 = x4;
    }
    return N;
}

// Find values that generate good randomness.
// OUT: 117 174 345 - GOOD
void find_p() {
    for (int p1 = 117; p1 < 1000; p1++)
        for (int p2 = 17; p2 < 1000; p2++)
            for (int p3 = 37; p3 < 1000; p3++) {
                cout << p1 << ' ' << p2 << ' ' << p3;
                int k = check(p1, p2, p3);
                if (k == N) {
                    cout << " - GOOD\n";
                    return;
                } else {
                    cout << " - BAD (" << k << ")\n";
                }
            }
}

void print_p(int p1, int p2, int p3) {
    int x1 = 0, x2 = 1, x3 = 2;
    putc('a' + x1, stdout);
    putc('a' + x2, stdout);
    putc('a' + x3, stdout);
    for (int i = 3; i < N; i++) {
        int x4 = next(x1, x2, x3, p1, p2, p3);
        putc('a' + x4, stdout);
        x1 = x2;
        x2 = x3;
        x3 = x4;
    }
}

// Iterate over all triads over and over again!
void easy_solution() {
    int len = 0;
    while (len < N)
        for (char a = 'a'; a <= 'z' && len < N; a++)
            for (char b = 'a'; b <= 'z' && len < N; b++)
                for (char c = 'a'; c <= 'z' && len < N; c++) {
                    if (len < N) { putc(a, stdout); len++; }
                    if (len < N) { putc(b, stdout); len++; }
                    if (len < N) { putc(c, stdout); len++; }
                }
}

int main() {
    //find_p();
    //print_p(117, 174, 345);
    easy_solution();
    return 0;
}
