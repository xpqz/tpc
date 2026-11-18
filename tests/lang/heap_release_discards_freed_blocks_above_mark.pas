program MarkFreeList;
type PRec = ^Rec; Rec = record x, y: LongInt; end;
var m: Pointer; a, b, c: PRec; before: LongInt;
begin
  New(c);
  Mark(m);
  before := MemAvail;
  New(a);
  New(b);
  Dispose(a);
  WriteLn(MemAvail < before);
  Release(m);
  WriteLn(MemAvail = before);
  New(a);
  New(b);
  a^.x := 1;
  b^.x := 2;
  WriteLn(a <> b, ' ', a^.x + b^.x);
  Dispose(c)
end.
