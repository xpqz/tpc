program ArithmeticPrecedence;
var a, b, c: Integer;
begin
  WriteLn(2 + 3 * 4);
  WriteLn((2 + 3) * 4);
  WriteLn(10 - 4 - 3);
  WriteLn(100 div 10 div 2);
  WriteLn(17 mod 5 * 2);
  WriteLn(-3 * -3);
  WriteLn(- 2 * 3);
  WriteLn(1 + 2 shl 3);
  WriteLn(7 and 3 + 1);
  WriteLn(8 - 2 - 1 - 1);
  a := 5;
  b := 3;
  c := 2;
  WriteLn(a + b * c - a div c);
  WriteLn((a + b) * (c - 1) mod 3);
  WriteLn(a - -b);
  WriteLn(-a + b)
end.
