import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    String str = in.nextLine().trim();
    int l = 0;
    char[] res = new char[100];
    for (int i = 0; i < str.length(); i++) {
      char c = str.charAt(i);
      if (l > 0 && res[l - 1] == c) l--;
      else res[l++] = c;
    }
    if (l > 0) System.out.println(new String(res, 0, l));
    else System.out.println("Empty String");
  }
}
