program StrNeg;
{$R+}
var s: String[3]; i: Integer;
begin
  s := 'abc';
  i := 0;
  WriteLn(Ord(s[i]));
  i := -1;
  WriteLn(Ord(s[i]));
  WriteLn('not reached')
end.
