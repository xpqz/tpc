program RealFuncs;
var x: Real; i: Integer; l: LongInt;
begin
  WriteLn(Sqrt(16.0):0:3, ' ', Sqrt(2.25):0:3, ' ', Sqrt(0.25):0:3, ' ', Sqrt(0.0):0:3);
  WriteLn(Sqr(1.5):0:3, ' ', Sqr(-2.5):0:3, ' ', Sqr(0.5):0:3);
  WriteLn(Abs(-2.5):0:3, ' ', Abs(2.5):0:3, ' ', Abs(0.0):0:3);
  x := 144;
  WriteLn(Sqrt(x):0:1);
  i := 5;
  WriteLn(Sqr(i), ' ', Abs(-i), ' ', Abs(i - 10));
  l := -70000;
  WriteLn(Sqr(l div 100), ' ', Abs(l), ' ', Sqr(300) );
  for i := 1 to 12 do Write(Round(Sqrt(i * i)), ' ');
  WriteLn;
  WriteLn(Sqrt(Sqr(7.0)):0:2, ' ', Sqr(Sqrt(81.0)):0:2)
end.
