program SetCopy;
var a, b: set of 0..40; i: Integer;
begin
  a := [1, 2, 3];
  b := a;
  b := b + [40];
  WriteLn(40 in a, ' ', 40 in b, ' ', a <= b, ' ', b <= a);
  a := a + a;
  a := a * a;
  a := a - [];
  WriteLn(a = [1, 2, 3]);
  a := a - a;
  WriteLn(a = [], ' ', b = [1, 2, 3, 40]);
  for i := 0 to 40 do b := b - [i];
  WriteLn(b = [])
end.
