program MarkBasic;
type PI = ^Integer;
var m: Pointer; before: LongInt; p: PI; i: Integer;
begin
  Mark(m);
  before := MemAvail;
  for i := 1 to 100 do begin
    New(p);
    p^ := i;
  end;
  WriteLn(MemAvail < before);
  Release(m);
  WriteLn(MemAvail = before)
end.
