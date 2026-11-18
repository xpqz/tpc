program FreeMemNil;
var p: Pointer;
begin
  WriteLn('before');
  p := nil;
  FreeMem(p, 4);
  WriteLn('not reached')
end.
