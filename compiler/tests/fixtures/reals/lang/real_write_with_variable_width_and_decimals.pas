program RealWriteVar;
var x: Real; w, d, i: Integer;
function Width: Integer;
begin
  Width := 9
end;
begin
  x := 12.40625;
  for w := 0 to 8 do
    WriteLn('[', x:w:2, ']');
  for d := 0 to 3 do
    WriteLn('[', x:10:d, ']');
  w := 3; d := 1;
  WriteLn('[', x:w + 4:d + 1, ']');
  WriteLn('[', x:Width:Width - 7, ']');
  WriteLn('[', (x * 2):w:d, ']');
  WriteLn('[', x / 4:0:4, ']');
  i := 5;
  WriteLn(i:4, x:8:1, i:3, 'z':2, x:7:2);
  Write(x:6:1); Write(' '); Write(x:0:0); WriteLn;
  Write(x:11);
  WriteLn;
  WriteLn(x:w + 10)
end.
