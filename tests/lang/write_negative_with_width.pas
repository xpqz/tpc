program WriteNegativeWithWidth;
var i: Integer;
begin
  WriteLn(-5:4);
  WriteLn(-5:2);
  WriteLn(-5:1);
  WriteLn(-123:5);
  WriteLn(-123:4);
  WriteLn(-123:3);
  WriteLn(-123:2);
  i := -32768;
  WriteLn(i:6);
  WriteLn(i:5);
  WriteLn(i:7);
  WriteLn(-1:2);
  WriteLn(-1:3);
  WriteLn(-2147483647:12)
end.
