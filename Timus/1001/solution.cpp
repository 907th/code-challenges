#include <cstdio>
#include <cmath>

const int N = 256 * 1024;

int main() {
  int n = 0;
  long long int a[N];
  while (scanf("%lld", &a[n]) != EOF) n++;
  for (int i = n - 1; i >= 0; i--) printf("%.4f\n", sqrt(a[i]));
  return 0;
}
