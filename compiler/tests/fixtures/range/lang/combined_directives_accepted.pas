program Combined;
{$R+,F+}
{$F-,R-}
var b: Byte; i: Integer;
begin
  i := 300;
  b := i;
  WriteLn(b);
  {$F+,R+}
  i := 200;
  b := i;
  WriteLn(b)
end.
