program OddAbsSqr;
var
  i: Integer;
  l: LongInt;
begin
  i := -7;
  WriteLn(Abs(i), ' ', Odd(i), ' ', Odd(i + 1));
  WriteLn(Abs(5), ' ', Abs(0));
  WriteLn(Sqr(12), ' ', Sqr(-9));
  l := 40000;
  WriteLn(Abs(-l), ' ', Odd(l), ' ', Sqr(l));
  WriteLn(Odd(0), ' ', Odd(1), ' ', Odd(-3))
end.
