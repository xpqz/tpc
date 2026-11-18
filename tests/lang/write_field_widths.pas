program WriteFieldWidths;
var n, w: Integer;
begin
  WriteLn(42:5);
  WriteLn(42:2);
  WriteLn(42:1);
  WriteLn(42:0);
  WriteLn(0:3);
  WriteLn(7:10);
  n := 12345;
  w := 8;
  WriteLn(n:w);
  WriteLn(n:w - 3);
  WriteLn(n:w div 2);
  Write(1:3, 2:3, 3:3);
  WriteLn;
  WriteLn(100000:9)
end.
