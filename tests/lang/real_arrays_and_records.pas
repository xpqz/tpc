program RealAggregates;
type
  Vec = array[1..5] of Real;
  Pt = record x, y: Real; end;
var v: Vec; k: Integer; sum, mean: Real; p, q: Pt; m: array[1..2, 1..2] of Real;
function Dist2(a, b: Pt): Real;
begin
  Dist2 := Sqr(a.x - b.x) + Sqr(a.y - b.y);
end;
begin
  for k := 1 to 5 do v[k] := k * 1.5;
  sum := 0;
  for k := 1 to 5 do sum := sum + v[k];
  mean := sum / 5;
  WriteLn(sum:0:2, ' ', mean:0:2);
  p.x := 1; p.y := 2;
  q := p;
  q.x := q.x + 3;
  q.y := q.y + 4;
  WriteLn(p.x:0:1, ' ', q.x:0:1, ' ', Dist2(p, q):0:1);
  m[1, 1] := 1; m[1, 2] := 0.5; m[2, 1] := 0.25; m[2, 2] := 2;
  WriteLn(m[1, 1] * m[2, 2] - m[1, 2] * m[2, 1]:0:4);
  WriteLn(SizeOf(Vec) = 5 * SizeOf(Real), ' ', SizeOf(Pt) = 2 * SizeOf(Real))
end.
