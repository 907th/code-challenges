#include <cstdio>
#include <cstring>

const char* CHARS[] = { "oqz", "ij", "abc", "def", "gh", "kl", "mn", "prs", "tuv", "wxy" };

int number_len, words_count;
char number[105];
char words[5005][55];

void solve() {
}

int main() {
  while (true) {
    scanf("%s%n", number, &number_len);
    number[number_len] = 0;
    if (strcmp(number, "-1") == 0) break;

    scanf("%d", &words_count);
    for (int i = 0; i < words_count; i++) scanf("%s", words[i]);

    /* printf("%s %d %d\n", number, number_len, words_count); */
    /* for (int i = 0; i < words_count; i++) printf("%s\n", words[i]); */

    solve();
  }
  return 0;
}
