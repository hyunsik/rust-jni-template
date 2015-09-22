package org.github.hyunsik;

import java.nio.file.Path;
import java.nio.file.Paths;

public class NativeInvoke {

  public static native void procedure();

  public static native void stringArg(String str);

  public static native String returnString();
}
