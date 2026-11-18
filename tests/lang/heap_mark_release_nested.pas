program MarkNested;
var m1, m2: Pointer; a, b, c: LongInt; p: ^LongInt; i: Integer;
begin
  Mark(m1);
  a := MemAvail;
  for i := 1 to 10 do New(p);
  Mark(m2);
  b := MemAvail;
  for i := 1 to 10 do New(p);
  c := MemAvail;
  WriteLn((a > b) and (b > c));
  Release(m2);
  WriteLn(MemAvail = b);
  Release(m1);
  WriteLn(MemAvail = a)
end.
