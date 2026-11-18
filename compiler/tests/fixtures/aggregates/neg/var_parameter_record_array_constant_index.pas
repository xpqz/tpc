program P;
type Rec = record a, b: Integer end;
var rv: array[1..2] of Rec;
procedure Q(var x: Integer);
begin
  x := 5
end;
begin
  Q(rv[1].b); Q(rv[3].a)
end.
