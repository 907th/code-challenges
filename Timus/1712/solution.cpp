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

int main() {
    char grille[4][4];
    char cipher[4][4];
    for (int i = 0; i < 4; i++) {
        char buf[5];
        gets(buf);
        memcpy(grille[i], buf, 4);
    }
    for (int i = 0; i < 4; i++) {
        char buf[5];
        gets(buf);
        memcpy(cipher[i], buf, 4);
    }
    int ans_len = 0;
    char answer[17];
    for (int i = 0; i < 4; i++) {
        for (int x = 0; x < 4; x++)
            for (int y = 0; y < 4; y++)
                if (grille[x][y] == 'X')
                    answer[ans_len++] = cipher[x][y];
        // Rotate
        char new_grille[4][4];
        for (int x = 0; x < 4; x++)
            for (int y = 0; y < 4; y++)
                new_grille[y][3 - x] = grille[x][y];
        memcpy(grille, new_grille, sizeof(grille));
    }
    answer[16] = 0;
    printf("%s\n", answer);
    return 0;
}
