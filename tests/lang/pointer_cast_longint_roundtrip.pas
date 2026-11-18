program PtrCastLong;
type PI = ^Integer;
var p, q: PI; l: LongInt;
begin
  WriteLn(LongInt(nil), ' ', LongInt(nil) = 0);
  p := nil;
  l := LongInt(p);
  WriteLn(l);
  New(p);
  p^ := 1234;
  l := LongInt(p);
  WriteLn(l <> 0);
  q := PI(l);
  WriteLn(q = p, ' ', q^);
  q^ := 99;
  WriteLn(p^);
  q := PI(LongInt(p));
  WriteLn(q = p);
  Dispose(p)
end.
