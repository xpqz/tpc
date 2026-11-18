program RangeSub;
{$R+}
var s: 1..10; i: Integer;
begin
  WriteLn('before');
  i := 11;
  s := i;
  WriteLn(s);
  WriteLn('not reached')
end.
