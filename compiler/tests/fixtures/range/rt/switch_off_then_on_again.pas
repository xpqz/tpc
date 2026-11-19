program SwitchOnOff;
var b: Byte; i: Integer;
begin
  i := 300;
  {$R+}
  b := 5;
  WriteLn('a ', b);
  {$R-}
  b := i;
  WriteLn('b ', b);
  {$R+}
  b := i;
  WriteLn('not reached ', b)
end.
