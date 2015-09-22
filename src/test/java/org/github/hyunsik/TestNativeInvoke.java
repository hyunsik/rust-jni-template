package org.github.hyunsik;

import org.junit.BeforeClass;
import org.junit.Test;
import static org.junit.Assert.*;

import java.nio.file.Path;
import java.nio.file.Paths;

public class TestNativeInvoke {
  @BeforeClass
  public static void setUp() {
    Path p = Paths.get("src/main/rust/target/debug/libnative.so");
    System.load(p.toAbsolutePath().toString());
  }

  @Test
  public final void testProcedure() {
    NativeInvoke.procedure();
  }

  @Test
  public final void testStringArg() {
    NativeInvoke.stringArg("XXX");
  }

  @Test
  public final void testReturnString() {
    assertEquals("jni native", NativeInvoke.returnString());
  }
}
