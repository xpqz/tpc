program PtrReuse;
type PI = ^Integer;
var a, b, c, d: PI;
begin
  New(a); New(b); New(c);
  a^ := 1; b^ := 2; c^ := 3;
  Dispose(b);
  Dispose(a);
  New(d);
  d^ := 4;
  WriteLn(c^, ' ', d^);
  New(a);
  New(b);
  a^ := 10;
  b^ := 20;
  WriteLn(a^ + b^ + c^ + d^);
  WriteLn((a <> b) and (a <> c) and (a <> d) and (b <> c) and (b <> d) and (c <> d))
end.
