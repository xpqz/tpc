program AggregateSp;
type VT = array[1..50] of Integer; S = record s: String; v: VT end;
function Sum(v: VT): Integer;
var i, t: Integer;
begin t := 0; for i := 1 to 50 do t := t + v[i]; Sum := t end;
function Name(s: S): String; begin Name := s.s end;
var v: VT; r: S; i, t: Integer;
begin
  for i := 1 to 50 do v[i] := i;
  r.s := 'rec'; r.v := v;
  t := 0;
  for i := 1 to 200 do t := t + Sum(v) + Length(Name(r));
  WriteLn(t)
end.
