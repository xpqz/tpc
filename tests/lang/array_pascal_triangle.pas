program Pascal;
var t: array[0..9, 0..9] of Integer; i, j: Integer;
begin
  for i := 0 to 9 do begin
    t[i, 0] := 1;
    for j := 1 to i do
      if j = i then t[i, j] := 1 else t[i, j] := t[i - 1, j - 1] + t[i - 1, j]
  end;
  for i := 0 to 9 do begin
    for j := 0 to i do Write(t[i, j], ' ');
    WriteLn
  end
end.
