program RangeLow;
{$R+}
var a: array[1..5] of Integer; i: Integer;
begin
  WriteLn('before');
  i := 0;
  a[i] := 1;
  WriteLn(a[1]);
  WriteLn('not reached')
end.
