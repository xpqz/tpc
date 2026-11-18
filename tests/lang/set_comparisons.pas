program SetCompare;
type TS = set of 1..10;
var a, b, c, e: TS;
begin
  a := [1, 2, 3];
  b := [1, 2, 3, 4];
  c := [1, 2, 3];
  e := [];
  WriteLn(a = c, ' ', a = b, ' ', a <> b, ' ', a <> c);
  WriteLn(a <= b, ' ', b <= a, ' ', a <= c, ' ', a >= c);
  WriteLn(b >= a, ' ', a >= b, ' ', e <= a, ' ', a >= e);
  WriteLn(e = [], ' ', e <> a, ' ', e <= e, ' ', e >= e);
  WriteLn(a = [1..3], ' ', b = [1..4], ' ', [1] <= a, ' ', [5] <= a);
  WriteLn([3, 2, 1] = a, ' ', [1, 2] >= [1], ' ', [2..4] <= [1..10]);
  a := [1, 5];
  b := [2, 6];
  WriteLn(a <= b, ' ', a >= b, ' ', a = b, ' ', a <> b)
end.
