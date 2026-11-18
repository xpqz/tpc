program RealSci;
var x: Real;
begin
  x := 1234.5;
  WriteLn(x:12);
  WriteLn(x:15);
  WriteLn(x:20);
  WriteLn(x:8);
  WriteLn(x:1);
  WriteLn(x:0);
  x := -0.125;
  WriteLn(x:10);
  WriteLn(x:16);
  x := 2.0;
  WriteLn(x:6);
  WriteLn('[', x:13, ']')
end.
