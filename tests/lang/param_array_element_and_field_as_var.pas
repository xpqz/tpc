program ParamElementVar;
type R = record a, b: Integer end;
var t: array[1..3] of R; v: array[1..3] of Integer; i: Integer;
procedure Inc2(var x: Integer); begin x := x + 2 end;
begin
  for i := 1 to 3 do begin v[i] := i; t[i].a := i; t[i].b := -i end;
  Inc2(v[2]); Inc2(t[3].b); Inc2(t[1].a);
  for i := 1 to 3 do Write(v[i], ' ', t[i].a, ' ', t[i].b, ' ');
  WriteLn
end.
