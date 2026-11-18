program ArrayBounds;
var a: array[-3..3] of Integer; b: array[100..102] of Byte; c: array[0..0] of Integer; i: Integer;
begin
  for i := -3 to 3 do a[i] := i * i;
  for i := -3 to 3 do Write(a[i], ' ');
  WriteLn;
  b[100] := 1; b[101] := 2; b[102] := 255;
  WriteLn(b[100] + b[101] + b[102]);
  c[0] := 42;
  WriteLn(c[0]);
  WriteLn(SizeOf(a), ' ', SizeOf(b), ' ', SizeOf(c))
end.
