program RealIntFrac;
var x: Real;
begin
  x := 2.75;
  WriteLn(Int(x):0:2, ' ', Frac(x):0:2);
  x := -2.75;
  WriteLn(Int(x):0:2, ' ', Frac(x):0:2);
  x := 0.125;
  WriteLn(Int(x):0:3, ' ', Frac(x):0:3);
  x := 1000.0;
  WriteLn(Int(x):0:1, ' ', Frac(x):0:1);
  WriteLn(Pi:0:5, ' ', Pi * 2:0:4, ' ', Trunc(Pi * 1000))
end.
