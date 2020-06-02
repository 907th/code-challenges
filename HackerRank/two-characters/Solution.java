import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);

    String strLen = in.nextLine().trim();
    String str = in.nextLine().trim();

    int n = Integer.parseInt(strLen);
    char[] s = str.toCharArray();

    int res = 0;
    for (char a = 'a'; a <= 'z'; a++) {
ITER: for (char b = 'a'; b <= 'z'; b++) {
        int l = 0;
        char[] w = new char[n];
        for (int i = 0; i < n; i++) {
          if (s[i] == a || s[i] == b) w[l++] = s[i];
          if (l > 1 && w[l - 1] == w[l - 2]) continue ITER;
        }
        if (l > 1) {
          // System.out.printf("%c %c: %s\n", a, b, new String(w, 0, l));
          if (res < l) res = l;
        }
      }
    }
    System.out.println(res);
  }
}
