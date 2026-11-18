program ParamVarArray;
type V = array[1..4] of Integer;
procedure Rev(var v: V);
var i, t: Integer;
begin
  for i := 1 to 2 do begin t := v[i]; v[i] := v[5 - i]; v[5 - i] := t end
end;
procedure Fill(var v: V; k: Integer);
var i: Integer;
begin
  for i := 1 to 4 do v[i] := k + i
end;
var a: V; i: Integer;
begin
  Fill(a, 10);
  Rev(a);
  for i := 1 to 4 do Write(a[i], ' ');
  WriteLn
end.
