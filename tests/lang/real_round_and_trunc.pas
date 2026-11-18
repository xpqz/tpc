program RealRoundTrunc;
var x: Real; l: LongInt;
begin
  WriteLn(Round(2.5), ' ', Round(-2.5), ' ', Round(3.5), ' ', Round(-3.5));
  WriteLn(Round(0.5), ' ', Round(-0.5), ' ', Round(1.5), ' ', Round(0.25), ' ', Round(0.75));
  WriteLn(Round(2.4999), ' ', Round(-2.4999), ' ', Round(2.0), ' ', Round(0.0));
  WriteLn(Trunc(2.7), ' ', Trunc(-2.7), ' ', Trunc(0.99), ' ', Trunc(-0.99), ' ', Trunc(5.0));
  x := 100000.5;
  l := Round(x);
  WriteLn(l);
  l := Trunc(x);
  WriteLn(l);
  x := -2147483000.25;
  WriteLn(Trunc(x), ' ', Round(x));
  x := 32767.5;
  WriteLn(Round(x), ' ', Trunc(x));
  x := 40000.75;
  WriteLn(Round(x) div 2, ' ', Trunc(x) mod 7);
  x := 1000000.0;
  WriteLn(Round(x) * 2)
end.
