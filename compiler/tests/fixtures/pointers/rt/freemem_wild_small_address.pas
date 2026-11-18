program FreeWild;
type PB = ^Byte;
var p: PB;
begin
  WriteLn('before');
  p := PB(LongInt(10));
  FreeMem(p, 4);
  WriteLn('not reached')
end.
