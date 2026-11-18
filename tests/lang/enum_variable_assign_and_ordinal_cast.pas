program EnumCast;
type T = (A, B, C, D, E);
var x, y: T; n: Integer;
begin
  n := 3;
  x := T(n);
  y := T(n - 3);
  WriteLn(Ord(x), ' ', Ord(y));
  x := E; y := x;
  WriteLn(Ord(y), ' ', y = E);
  n := Ord(x) * 100 + Ord(y);
  WriteLn(n)
end.
