program AggregateExit;
type VT = array[1..8] of Integer;
function Find(v: VT; k: Integer): Integer;
var i: Integer;
begin
  Find := 0;
  for i := 1 to 8 do
    if v[i] = k then begin Find := i; Exit end
end;
procedure Clobber(v: VT; var hit: Integer);
begin
  v[1] := 0;
  if hit > 0 then Exit;
  hit := 1
end;
var v: VT; i, h: Integer;
begin
  for i := 1 to 8 do v[i] := i * 3;
  WriteLn(Find(v, 12), Find(v, 99), Find(v, 3));
  h := 0; Clobber(v, h); Clobber(v, h);
  WriteLn(h, v[1])
end.
