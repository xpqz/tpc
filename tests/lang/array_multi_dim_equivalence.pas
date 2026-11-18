program ArrayMultiEquiv;
var a: array[1..3, 0..2] of Integer; b: array[1..3] of array[0..2] of Integer; i, j: Integer;
begin
  for i := 1 to 3 do
    for j := 0 to 2 do begin
      a[i][j] := i * 100 + j;
      b[i, j] := i * 100 + j
    end;
  for i := 1 to 3 do
    for j := 0 to 2 do
      if (a[i, j] <> b[i][j]) or (a[i][j] <> b[i, j]) then WriteLn('diff ', i, ' ', j);
  WriteLn(a[2, 1], ' ', a[2][1], ' ', b[3][2], ' ', b[3, 2]);
  WriteLn(SizeOf(a), ' ', SizeOf(b))
end.
