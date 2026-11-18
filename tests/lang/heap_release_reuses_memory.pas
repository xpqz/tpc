program MarkReuse;
type PI = ^Integer;
var m: Pointer; a, b: PI;
begin
  Mark(m);
  New(a);
  a^ := 5;
  Release(m);
  New(b);
  WriteLn(a = b);
  b^ := 6;
  WriteLn(b^)
end.
