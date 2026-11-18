program ParamAlias;
type VT = array[1..3] of Integer;
procedure P(a: VT; var b: VT);
begin
  b[1] := 100;
  WriteLn(a[1], ' ', b[1])
end;
var v: VT;
begin
  v[1] := 1;
  P(v, v);
  WriteLn(v[1])
end.
