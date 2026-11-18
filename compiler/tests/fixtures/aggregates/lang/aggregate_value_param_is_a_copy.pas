program AggregateValueParamIsACopy;
type Vec = array[1..3] of Integer; Pt = record a, b: Integer; v: Vec end;
var gv: Vec; gp: Pt;
function Sum(v: Vec): Integer;
begin
  Sum := v[1] + v[2] + v[3]
end;
procedure Scribble(v: Vec; p: Pt);
begin
  v[1] := 99; p.a := 77; p.v[3] := 55;
  gv[2] := v[1] + p.a
end;
procedure Twice(v: Vec);
  procedure Inner(w: Vec);
  begin
    w[1] := w[1] * 2;
    v[1] := w[1];
    WriteLn(w[1], ' ', v[1])
  end;
begin
  Inner(v);
  WriteLn(v[1], ' ', Sum(v))
end;
function Depth(v: Vec; n: Integer): Integer;
begin
  v[1] := v[1] + n;
  if n = 0 then Depth := v[1] else Depth := Depth(v, n - 1) + v[1]
end;
begin
  gv[1] := 1; gv[2] := 2; gv[3] := 3;
  gp.a := 4; gp.b := 5; gp.v := gv;
  Scribble(gv, gp);
  WriteLn(gv[1], gv[2], gv[3], ' ', gp.a, gp.b, gp.v[1], gp.v[2], gp.v[3]);
  Twice(gv);
  WriteLn(gv[1], ' ', Depth(gv, 3), ' ', gv[1]);
  WriteLn(Sum(gv), ' ', Sum(gp.v))
end.
