import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int n = in.nextInt();
    int maxv = Integer.MIN_VALUE, cnt = 0;
    for (int i = 0; i < n; i++) {
      int v = in.nextInt();
      if (v > maxv) {
        maxv = v;
        cnt = 1;
      } else if (v == maxv) {
        cnt++;
      }
    }
    System.out.println(cnt);
  }
}
