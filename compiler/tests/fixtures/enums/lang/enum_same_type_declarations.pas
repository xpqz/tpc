program EnumSameTypeDeclarations;
type A = (x1, x2, x3); B = A;
var p, q: (u, v, w); r: A; s: B;
procedure Take(var a: A);
begin
  a := x3
end;
begin
  p := w; q := p;
  WriteLn(Ord(q), Ord(p = q));
  r := x2; s := r;
  Take(s); Take(r);
  WriteLn(Ord(r), Ord(s))
end.
