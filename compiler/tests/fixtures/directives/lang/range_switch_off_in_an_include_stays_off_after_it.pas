program RangeSwitchOffInAnInclude;
{$R+}
var
  b: Byte;
  i: Integer;
{$I OFF.INC}
begin
  i := 300;
  b := i;
  WriteLn(b);
end.
