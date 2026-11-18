program HeapSizes;
type
  R = record a: Byte; b: LongInt; c: Char; end;
  PR = ^R;
var pb: ^Byte; pl: ^LongInt; prc: PR; ps: ^String; before: LongInt;
begin
  before := MemAvail;
  New(pb); New(pl); New(prc); New(ps);
  WriteLn(before - MemAvail >= 1 + 4 + 6 + 256);
  pb^ := 255; pl^ := -70000; prc^.a := 1; prc^.b := 2; prc^.c := 'z'; ps^ := 'str';
  WriteLn(pb^, ' ', pl^, ' ', prc^.b, prc^.c, ' ', ps^);
  Dispose(ps); Dispose(prc); Dispose(pl); Dispose(pb);
  WriteLn(MemAvail = before)
end.
