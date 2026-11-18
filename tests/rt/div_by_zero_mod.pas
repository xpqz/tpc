program ModZero;
var i, z: Integer;
begin
  WriteLn('before');
  z := 0;
  i := 10 mod z;
  WriteLn(i);
  WriteLn('not reached')
end.
