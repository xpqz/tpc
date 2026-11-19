program RtRealZero;
var x, y: Real;
begin
  x := 2.5; y := 0;
  WriteLn((x / 2):0:2);
  WriteLn((y / x):0:2);
  Write('before ');
  y := x / y;
  WriteLn('not reached')
end.
