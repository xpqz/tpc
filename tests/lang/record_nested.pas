program RecordNested;
type
  Pt = record x, y: Integer end;
  Rect = record a, b: Pt; id: Byte end;
  Pair = record r: Rect; n: LongInt end;
var r: Rect; p: Pair;
begin
  r.a.x := 1; r.a.y := 2; r.b.x := 3; r.b.y := 4; r.id := 9;
  p.r := r; p.n := 100000;
  p.r.b.y := 44;
  WriteLn(r.b.y, ' ', p.r.b.y, ' ', p.r.a.x + p.r.b.x, ' ', p.n);
  WriteLn(SizeOf(Pt), ' ', SizeOf(Rect), ' ', SizeOf(Pair))
end.
