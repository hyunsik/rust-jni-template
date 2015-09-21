package org.github.hyunsik;

import java.nio.file.Path;
import java.nio.file.Paths;

public class NativeMain {
  public static native void helloJre();

  public static void main(String [] args) {
    Path p = Paths.get("src/main/rust/target/debug/libnative.so");
    System.load(p.toAbsolutePath().toString());
    NativeMain.helloJre();
  }
}
