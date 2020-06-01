import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);

    String time = in.nextLine();

    int hh = Integer.parseInt(time.substring(0, 2));
    int mm = Integer.parseInt(time.substring(3, 5));
    int ss = Integer.parseInt(time.substring(6, 8));
    String am = time.substring(8, 10);

    if (am.equals("AM") && hh == 12) hh = 0;

    if (am.equals("PM")) {
      if (hh != 12) hh += 12;
    }

    System.out.printf("%02d:%02d:%02d\n", hh, mm, ss);
  }
}
