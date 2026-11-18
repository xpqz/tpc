program ArrayRowAssign;
type Row = array[1..3] of Integer;
var m: array[1..3] of Row; r: Row; i, j: Integer;
begin
  for i := 1 to 3 do for j := 1 to 3 do m[i][j] := i * j;
  r := m[2];
  r[1] := 99;
  m[3] := r;
  m[1] := m[2];
  for i := 1 to 3 do begin
    for j := 1 to 3 do Write(m[i][j]:3);
    WriteLn
  end;
  WriteLn(r[1], ' ', r[2], ' ', r[3])
end.
