program WordNeg;
{$R+}
var w: Word; i: Integer;
begin
  i := -1;
  w := 5;
  WriteLn('before ', w);
  w := i;
  WriteLn('not reached')
end.
