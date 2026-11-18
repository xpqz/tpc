program RealCompare;
var a, b: Real; i: Integer;
begin
  a := 2.5;
  b := 2.5;
  i := 2;
  WriteLn(a = b, ' ', a <> b, ' ', a < b, ' ', a <= b, ' ', a > b, ' ', a >= b);
  b := 3.0;
  WriteLn(a = b, ' ', a <> b, ' ', a < b, ' ', a <= b, ' ', a > b, ' ', a >= b);
  WriteLn(a > i, ' ', i < a, ' ', i = 2.0, ' ', b = 3, ' ', 3 = b, ' ', 2.5 > 2);
  WriteLn(-a < 0, ' ', Abs(-a) = a, ' ', a + 0.5 = b, ' ', a * 2 > 4.99);
  WriteLn(0.5 + 0.25 = 0.75, ' ', 1.0E3 = 1000, ' ', 0.1 < 0.2)
end.
