program IntLong;
{$R+}
var i: Integer; l: LongInt;
begin
  l := 32767;
  i := l;
  WriteLn(i);
  l := 40000;
  i := l;
  WriteLn('not reached')
end.
