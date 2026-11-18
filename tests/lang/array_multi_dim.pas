program ArrayMultiDim;
var m: array[1..3, 1..4] of Integer; i, j: Integer;
begin
  for i := 1 to 3 do
    for j := 1 to 4 do
      m[i, j] := i * 10 + j;
  for i := 1 to 3 do begin
    for j := 1 to 4 do Write(m[i, j]:4);
    WriteLn
  end;
  WriteLn(SizeOf(m), ' ', m[3, 4], ' ', m[1, 1])
end.
