program RealAggregates;
type
  Vec = array[1..4] of Real;
  Pt = record x, y: Real; tag: Integer end;
var
  v: Vec; m: array[1..2, 1..3] of Real; p, q: Pt; i, j: Integer; s: Real;

function Power(b: Real; n: Integer): Real;
begin
  if n = 0 then Power := 1
  else if n < 0 then Power := 1 / Power(b, -n)
  else Power := b * Power(b, n - 1)
end;

function Sum(var a: Vec): Real;
var k: Integer; t: Real;
begin
  t := 0;
  for k := 1 to 4 do t := t + a[k];
  Sum := t
end;

procedure Swap(var a, b: Real);
var t: Real;
begin
  t := a; a := b; b := t
end;

procedure Scale(var a: Vec; f: Real);
var k: Integer;
begin
  for k := 1 to 4 do a[k] := a[k] * f
end;

function Dist2(a, b: Pt): Real;
begin
  Dist2 := Sqr(a.x - b.x) + Sqr(a.y - b.y)
end;

begin
  for i := 1 to 4 do v[i] := i / 4;
  WriteLn(Sum(v):0:3);
  Scale(v, 8);
  for i := 1 to 4 do Write(v[i]:5:1);
  WriteLn;
  Swap(v[1], v[4]);
  WriteLn(v[1]:0:1, ' ', v[4]:0:1);
  for i := 1 to 2 do for j := 1 to 3 do m[i, j] := i * 10 + j / 2;
  s := 0;
  for i := 1 to 2 do for j := 1 to 3 do s := s + m[i, j];
  WriteLn(s:0:2, ' ', m[2, 3]:0:2);
  p.x := 3; p.y := 4; p.tag := 1;
  q.x := 0; q.y := 0; q.tag := 2;
  WriteLn(Dist2(p, q):0:1, ' ', Sqrt(Dist2(p, q)):0:1);
  q := p; q.x := q.x + 0.5;
  WriteLn(p.x:0:1, ' ', q.x:0:1, ' ', q.tag);
  WriteLn(Power(2, 10):0:0, ' ', Power(2, -3):0:5, ' ', Power(0.5, 4):0:4, ' ', Power(10, 6):0:0);
  Swap(p.x, p.y);
  WriteLn(p.x:0:0, p.y:2:0)
end.
