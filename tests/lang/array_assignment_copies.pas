program ArrayAssign;
var a, b: array[1..5] of Integer; i: Integer;
begin
  for i := 1 to 5 do a[i] := i;
  b := a;
  a[3] := 100;
  b[5] := -1;
  for i := 1 to 5 do Write(a[i], ' ');
  WriteLn;
  for i := 1 to 5 do Write(b[i], ' ');
  WriteLn;
  a := b;
  b[1] := 0;
  WriteLn(a[1], ' ', a[5])
end.
