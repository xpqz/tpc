program MatMul;
type M = array[1..3, 1..3] of Integer;
var a, b, c: M; i, j, k: Integer;
begin
  for i := 1 to 3 do for j := 1 to 3 do begin a[i, j] := i + j; b[i, j] := i - j end;
  for i := 1 to 3 do for j := 1 to 3 do begin
    c[i, j] := 0;
    for k := 1 to 3 do c[i, j] := c[i, j] + a[i, k] * b[k, j]
  end;
  for i := 1 to 3 do begin for j := 1 to 3 do Write(c[i, j]:5); WriteLn end
end.
