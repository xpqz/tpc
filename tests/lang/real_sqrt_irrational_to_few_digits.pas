program RealSqrt;
var i: Integer;
begin
  for i := 2 to 10 do Write(Sqrt(i):0:4, ' ');
  WriteLn;
  WriteLn(Sqrt(2):12:6);
  WriteLn(Sqrt(1000000.0 + 0.5):0:3);
  WriteLn(Sqrt(0.5):0:5)
end.
