program QueensCounts;
var col: array[1..10] of Integer; sols, n: Integer;
procedure Place(r: Integer);
var c, i: Integer; ok: Boolean;
begin
  if r > n then sols := sols + 1
  else for c := 1 to n do begin
    ok := True;
    for i := 1 to r - 1 do
      if (col[i] = c) or (Abs(col[i] - c) = r - i) then ok := False;
    if ok then begin col[r] := c; Place(r + 1) end
  end
end;
begin
  for n := 1 to 7 do begin
    sols := 0; Place(1);
    Write(sols, ' ')
  end;
  WriteLn
end.
