import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int n = in.nextInt();
    char[] s = new char[n];
    Arrays.fill(s, ' ');
    for (int i = 0; i < n; i++) {
      s[n - i - 1] = '#';
      System.out.println(s);
    }
  }
}
