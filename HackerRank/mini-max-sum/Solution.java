import java.io.*;
import java.util.*;
import static java.lang.Math.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    long sum = 0;
    long minv = Long.MAX_VALUE;
    long maxv = Long.MIN_VALUE;
    for (int i = 0; i < 5; i++) {
      long v = in.nextLong();
      sum += v;
      minv = min(minv, v);
      maxv = max(maxv, v);
    }
    System.out.printf("%d %d\n", sum - maxv, sum - minv);
  }
}
