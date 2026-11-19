program PtrTarget;
{$R+}
var p: ^Byte; i: Integer;
begin
  New(p);
  i := 255;
  p^ := i;
  WriteLn(p^);
  i := -1;
  p^ := i;
  WriteLn('not reached')
end.
