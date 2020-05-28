import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int n = in.nextInt();
    int plus, minus, zero;
    plus = minus = zero = 0;
    for (int i = 0; i < n; i++) {
      int v = in.nextInt();
      if (v < 0) minus++;
      else if (v > 0) plus++;
      else zero++;
    }
    System.out.printf("%.6f\n", plus * 1.0 / n);
    System.out.printf("%.6f\n", minus * 1.0 / n);
    System.out.printf("%.6f\n", zero  * 1.0 / n);
  }
}
