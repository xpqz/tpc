program RangePtrArr;
{$R+}
type PA = ^TA; TA = array[1..10] of Integer;
var p: PA; i: Integer;
begin
  WriteLn('before');
  New(p);
  i := 11;
  p^[i] := 1;
  WriteLn(p^[1]);
  WriteLn('not reached')
end.
