program IntFuncs;
var i: Integer; l: LongInt; b: Byte;
begin
  WriteLn(Odd(1), ' ', Odd(2), ' ', Odd(0), ' ', Odd(-1), ' ', Odd(-2), ' ', Odd(32767), ' ', Odd(-32768));
  l := 100001;
  WriteLn(Odd(l), ' ', Odd(l + 1), ' ', Odd(-l));
  b := 255;
  WriteLn(Odd(b), ' ', Odd(b - 1));
  i := -7;
  WriteLn(Abs(i), ' ', Abs(7), ' ', Abs(0), ' ', Abs(-32767));
  l := -2000000000;
  WriteLn(Abs(l), ' ', Abs(l + 1));
  WriteLn(Sqr(7), ' ', Sqr(-7), ' ', Sqr(0), ' ', Sqr(181));
  l := 40000;
  WriteLn(Sqr(l div 100));
  WriteLn(Ord('A'), ' ', Chr(66), ' ', Succ(5), ' ', Pred(5), ' ', Succ('a'), ' ', Pred('b'))
end.
