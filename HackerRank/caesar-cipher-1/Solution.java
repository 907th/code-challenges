import java.io.*;
import java.util.*;

public class Solution {
  static char caesar(char c, int k) {
    if ('A' <= c && c <= 'Z') return (char) ('A' + (c + k - 'A') % 26);
    if ('a' <= c && c <= 'z') return (char) ('a' + (c + k - 'a') % 26);
    return c;
  }

  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);

    int n = in.nextInt();
    in.nextLine();
    String s = in.nextLine();
    int k = in.nextInt();

    char[] w = new char[n];
    for (int i = 0; i < n; i++) {
      w[i] = caesar(s.charAt(i), k);
    }

    System.out.println(new String(w));
  }
}
