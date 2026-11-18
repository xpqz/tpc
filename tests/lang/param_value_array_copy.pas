program ParamValueArray;
type V = array[1..4] of Integer;
var a: V;
function Sum(v: V): Integer;
var i, s: Integer;
begin
  s := 0;
  for i := 1 to 4 do begin s := s + v[i]; v[i] := 0 end;
  Sum := s
end;
procedure Scribble(v: V);
begin
  v[1] := 99; v[4] := -99
end;
var i: Integer;
begin
  for i := 1 to 4 do a[i] := i * 10;
  WriteLn(Sum(a));
  Scribble(a);
  for i := 1 to 4 do Write(a[i], ' ');
  WriteLn(Sum(a))
end.
