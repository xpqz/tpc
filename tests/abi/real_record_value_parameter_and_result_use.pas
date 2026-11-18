program RealRecParam;
type V = record x, y, z: Real; n: Integer; end;
function Dot(a, b: V): Real;
begin
  Dot := a.x * b.x + a.y * b.y + a.z * b.z;
end;
procedure Scale(var a: V; f: Real);
begin
  a.x := a.x * f; a.y := a.y * f; a.z := a.z * f;
  a.n := a.n + 1;
end;
function Len2(a: V): Real;
begin
  Scale(a, 2);
  Len2 := Dot(a, a);
end;
var a, b: V;
begin
  a.x := 1; a.y := 2; a.z := 3; a.n := 0;
  b := a;
  b.x := -1;
  WriteLn(Dot(a, b):0:1);
  WriteLn(Len2(a):0:1, ' ', a.n, ' ', a.x:0:1);
  Scale(a, 0.5);
  WriteLn(a.x:0:2, ' ', a.n)
end.
