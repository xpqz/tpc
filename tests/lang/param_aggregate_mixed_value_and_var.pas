program ParamMixed;
type V = array[1..3] of Integer; R = record n: Integer; v: V end;
procedure Both(a: V; var b: V; r: R; var s: R);
var i: Integer;
begin
  for i := 1 to 3 do begin a[i] := a[i] + 1; b[i] := b[i] + 1; r.v[i] := 0; s.v[i] := s.v[i] * 2 end;
  r.n := 0; s.n := s.n + 1
end;
var x, y: V; p, q: R; i: Integer;
begin
  for i := 1 to 3 do begin x[i] := i; y[i] := i; p.v[i] := i; q.v[i] := i end;
  p.n := 1; q.n := 1;
  Both(x, y, p, q);
  for i := 1 to 3 do Write(x[i], y[i], p.v[i], q.v[i], ' ');
  WriteLn(p.n, q.n)
end.
