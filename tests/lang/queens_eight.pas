program Queens;
const N = 8;
var col: array[1..N] of Integer; sols: Integer;
function Safe(r, c: Integer): Boolean;
var i: Integer;
begin
  Safe := True;
  for i := 1 to r - 1 do
    if (col[i] = c) or (Abs(col[i] - c) = r - i) then Safe := False
end;
procedure Place(r: Integer);
var c, i: Integer;
begin
  if r > N then begin
    sols := sols + 1;
    if sols = 1 then begin
      for i := 1 to N do Write(col[i], ' ');
      WriteLn
    end
  end else
    for c := 1 to N do
      if Safe(r, c) then begin col[r] := c; Place(r + 1) end
end;
begin
  sols := 0;
  Place(1);
  WriteLn(sols)
end.
