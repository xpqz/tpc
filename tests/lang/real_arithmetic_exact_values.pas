program RealArith;
var a, b: Real;
begin
  a := 6.5;
  b := 2.25;
  WriteLn(a + b);
  WriteLn(a - b);
  WriteLn(a * b);
  WriteLn(a / b * b);
  WriteLn(a / 2);
  WriteLn(-a);
  WriteLn(a + b:10:3);
  WriteLn(a - b:10:3);
  WriteLn(a * b:10:4);
  WriteLn(a / 4:10:4);
  WriteLn(-b * -b:10:5)
end.
