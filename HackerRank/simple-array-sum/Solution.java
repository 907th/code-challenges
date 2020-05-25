import java.io.*;
import java.util.*;
import java.text.*;
import java.math.*;
import java.util.regex.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int n, sum = 0;
    n = in.nextInt();
    for (int i = 0; i < n; i++) {
      int a = in.nextInt();
      sum += a;
    }
    System.out.println(sum);
  }
}
