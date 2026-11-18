program ArrayThreeDim;
var c: array[0..1, 0..2, 0..3] of Byte; i, j, k, s: Integer;
begin
  for i := 0 to 1 do for j := 0 to 2 do for k := 0 to 3 do c[i, j, k] := i * 12 + j * 4 + k;
  s := 0;
  for i := 0 to 1 do for j := 0 to 2 do for k := 0 to 3 do s := s + c[i, j, k];
  WriteLn(s, ' ', c[1, 2, 3], ' ', c[1][2][3], ' ', c[0, 1][2], ' ', SizeOf(c))
end.
