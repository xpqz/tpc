program ArgWord;
{$R+}
var i: Integer;
procedure P(w: Word);
begin
  WriteLn(w);
end;
begin
  i := 7;
  P(i);
  i := -7;
  P(i);
  WriteLn('not reached')
end.
