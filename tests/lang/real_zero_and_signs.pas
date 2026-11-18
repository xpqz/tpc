program RealZero;
var z, x: Real;
begin
  z := 0;
  x := 5;
  WriteLn(z);
  WriteLn(z:0:0, ' ', z:6:3);
  WriteLn(x * z:0:2, ' ', z / x:0:2, ' ', z - z:0:1);
  WriteLn(z = 0, ' ', -z = 0, ' ', x * z = z);
  WriteLn(Round(z), ' ', Trunc(-z), ' ', Abs(z) = 0, ' ', Sqrt(z):0:1)
end.
