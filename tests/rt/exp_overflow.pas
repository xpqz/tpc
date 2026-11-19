program ExpOverflow;
var x: Real;
begin
  Write(Exp(10):0:1, ' ');
  x := 1000;
  Write(Exp(x):0:1);
  WriteLn('not reached');
end.
