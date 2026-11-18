program RealWidth;
var x: Real;
begin
  x := 3.75;
  WriteLn(x:0:0);
  WriteLn(x:1:1);
  WriteLn(x:6:2);
  WriteLn(x:10:4);
  WriteLn(x:3:2);
  WriteLn(x:2:0);
  WriteLn('[', x:12:3, ']');
  x := -3.75;
  WriteLn(x:0:2);
  WriteLn(x:8:1);
  WriteLn('[', x:4:2, ']');
  x := 0;
  WriteLn(x:5:2);
  WriteLn(x:0:0);
  x := 1024;
  WriteLn(x:0:0, '|', x:7:0, '|', x:7:1, '|', x:3:3);
  x := 0.5;
  WriteLn(x:0:1, '|', x:4:1, '|', x:8:6);
  x := 123456.25;
  WriteLn(x:0:2, '|', x:12:3)
end.
