program StrWrite;
{$R+}
var s: String[3]; i: Integer;
begin
  s := 'abc';
  i := 3;
  s[i] := 'z';
  WriteLn(s);
  i := 4;
  s[i] := 'y';
  WriteLn('not reached')
end.
