program RealFuncs;
var x: Real; i: Integer; l: LongInt;
begin
  WriteLn(Sqrt(0):0:1, ' ', Sqrt(1):0:1, ' ', Sqrt(4):0:1, ' ', Sqrt(2.25):0:2, ' ', Sqrt(1.0E10):0:1);
  WriteLn(Sqrt(0.0625):0:3, ' ', Sqrt(16 * 16):0:1);
  WriteLn(Sqr(3):0, ' ', Sqr(0.5):0:2, ' ', Sqr(-1.5):0:2, ' ', Sqr(1.0E3):0:0);
  WriteLn(Abs(-3), ' ', Abs(3), ' ', Abs(-2.5):0:1, ' ', Abs(2.5):0:1, ' ', Abs(0.0):0:1);
  i := -4; l := -70000; x := -0.125;
  WriteLn(Abs(i), ' ', Abs(l), ' ', Abs(x):0:3, ' ', Sqr(i), ' ', Sqr(x):0:5);
  WriteLn(Sqrt(Sqr(3.0) + Sqr(4.0)):0:1);
  WriteLn(Sqrt(Abs(x * -2)):0:2);
  x := Sqr(i) + Abs(i);
  WriteLn(x:0:1);
  x := Sqrt(Sqr(x));
  WriteLn(x:0:1);
  WriteLn(Sqrt(i * i):0:1);
  WriteLn(Sqr(Sqrt(9.0)):0:1)
end.
