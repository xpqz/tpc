program RealPromotion;
type TRec = record a: Real; n: Integer end;
var
  i: Integer; w: Word; b: Byte; l: LongInt; sh: ShortInt;
  r: Real; arr: array[1..3] of Real; rec: TRec;

procedure Show(x: Real);
begin
  WriteLn(x:0:2)
end;

function Twice(x: Real): Real;
begin
  Twice := x * 2
end;

function FromInt(n: Integer): Real;
begin
  FromInt := n * 3
end;

function Half(n: LongInt): Real;
begin
  Half := n / 2
end;

begin
  i := -7; w := 65000; b := 200; l := 100000; sh := -100;
  Show(i);
  Show(w);
  Show(b);
  Show(l);
  Show(sh);
  Show(i + 1);
  Show(3);
  Show(i * 100);
  Show(Twice(i));
  Show(Twice(b));
  Show(Twice(1.5));
  Show(FromInt(i));
  Show(FromInt(10000));
  Show(Half(l));
  Show(Half(-3));
  r := i; WriteLn(r:0:1);
  r := l; WriteLn(r:0:1);
  r := w + 1; WriteLn(r:0:1);
  r := b * 2; WriteLn(r:0:1);
  arr[1] := i; arr[2] := b; arr[3] := l;
  WriteLn(arr[1]:0:0, ' ', arr[2]:0:0, ' ', arr[3]:0:0);
  rec.a := w; rec.n := i;
  WriteLn(rec.a:0:0, ' ', rec.n);
  rec.a := rec.n;
  WriteLn(rec.a:0:0);
  Show(Twice(Twice(Twice(i))))
end.
