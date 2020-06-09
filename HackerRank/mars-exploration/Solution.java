import java.io.*;
import java.util.*;

public class Solution {
  static char[] sos = "SOS".toCharArray();

  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    String s = in.nextLine();
    int res = 0;
    for (int i = 0; i < s.length(); i++) {
      if (sos[i % 3] != s.charAt(i)) res++;
    }
    System.out.println(res);
  }
}
