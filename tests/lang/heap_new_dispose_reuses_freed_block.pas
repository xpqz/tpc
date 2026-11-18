program HeapReuse;
type PRec = ^Rec; Rec = record x, y: LongInt; end;
var a, b: PRec;
begin
  New(a);
  Dispose(a);
  New(b);
  WriteLn(a = b);
  Dispose(b)
end.
