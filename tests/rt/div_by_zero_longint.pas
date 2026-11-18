program DivLong;
var l, z: LongInt;
begin
  WriteLn('before');
  z := 5 - 5;
  l := 100000 div z;
  WriteLn(l);
  WriteLn('not reached')
end.
