program RangeSwitchInAnInclude;
var
  b: Byte;
  i: Integer;
{$I ON.INC}
begin
  i := 300;
  WriteLn('before');
  b := i;
  WriteLn(b);
end.
