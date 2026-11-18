program Casts;
type PI = ^Integer; PW = ^Word; PB = ^Byte; PL = ^LongInt;
var a, b: PI; l: LongInt; w: Word; vw: PW; vb: PB; vl: PL; n: PI;
begin
  n := nil;
  WriteLn(LongInt(n));
  New(a); New(b);
  a^ := 7;
  l := LongInt(a);
  WriteLn(l <> 0, ' ', LongInt(a) <> LongInt(b), ' ', LongInt(a) = l);
  WriteLn(PI(l) = a, ' ', PI(l)^);
  PI(l)^ := 8;
  WriteLn(a^);
  l := LongInt(b) - LongInt(a);
  WriteLn((l >= 2) or (l <= -2));
  b := PI(LongInt(a) + 0);
  WriteLn(b = a);
  vb := PB(a);
  w := 0;
  a^ := $1234;
  WriteLn(vb^, ' ', PB(LongInt(a) + 1)^);
  vw := PW(a);
  WriteLn(vw^);
  vl := PL(a);
  Dispose(a)
end.
