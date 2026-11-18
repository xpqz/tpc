program PtrCastBytes;
type PB = ^Byte;
var w: Word; p: PB; l: LongInt;
begin
  w := $1234;
  p := PB(@w);
  WriteLn(p^, ' ', Lo(w));
  p^ := $FF;
  WriteLn(w);
  l := $01020304;
  p := PB(@l);
  WriteLn(p^);
  p := PB(LongInt(@l) + 3);
  WriteLn(p^);
  p^ := 0;
  WriteLn(l)
end.
