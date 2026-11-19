program StaysOn;
var b: Byte; i: Integer;
procedure P;
begin
  {$R+}
  WriteLn('in P');
end;
begin
  WriteLn('before');
  i := 300;
  b := i;
  WriteLn('not reached ', b)
end.
