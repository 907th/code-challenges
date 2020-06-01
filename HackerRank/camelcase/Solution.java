import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    String s = in.nextLine().trim();
    int res = 0;
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (Character.isUpperCase(c)) res++;
    }
    System.out.println(res + 1);
  }
}
