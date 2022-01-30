#include <cstdio>

int main() {
    int n;
    scanf("%d", &n);

    // Moore's algo (MJRTY) with O(n) runtime
    int maj, num, k = 1;
    scanf("%d", &maj);
    for (int i = 1; i < n; i++) {
        scanf("%d", &num);
        if (num == maj) k++;
        else k--;
        if (k == 0) {
            maj = num;
            k = 1;
        }
    }
    // We must check that maj is actually a majority element now.
    // But it is guaranteed in this task that majority element exists!
    printf("%d\n", maj);

    return 0;
}
