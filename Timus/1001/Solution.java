import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) throws Exception {
    StreamTokenizer in = new StreamTokenizer(new BufferedReader(new InputStreamReader(System.in)));
    PrintWriter out = new PrintWriter(System.out);

    long a[] = new long[256 * 1024];
    int i = 0;

    while (in.nextToken() != StreamTokenizer.TT_EOF) {
      if (in.ttype != StreamTokenizer.TT_NUMBER) throw new Exception("Number expected!");
      a[i++] = (long)in.nval;
    }

    while (--i >= 0) {
      String res = String.format("%.4f", Math.sqrt(a[i]));
      out.println(res);
    }
    out.flush();
  }
}
