import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int[] colors = new int[100];
    int n = in.nextInt();
    for (int i = 0; i < n; i++) {
      int a = in.nextInt();
      colors[a - 1]++;
    }
    int res = 0;
    for (int i = 0; i < 100; i++) {
      res += colors[i] / 2;
    }
    System.out.println(res);
  }
}
