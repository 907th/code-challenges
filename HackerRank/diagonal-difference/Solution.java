import java.util.*;
import static java.lang.Math.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int n = in.nextInt();
    long a = 0, b = 0;
    for (int r = 0; r < n; r++) {
      for (int c = 0; c < n; c++) {
        int x = in.nextInt();
        if (c == r) a += x;
        if (c == n - r - 1) b += x;
      }
    }
    System.out.println(abs(a - b));
  }
}
