program RangeSwitchInAUnit;
uses Checked;
var
  b: Byte;
  i: Integer;
begin
  i := 300;
  b := i;
  WriteLn(b);
  WriteLn(Narrow(300));
end.
