program ForEqualBounds;
var
  i: Integer;
  c: Char;
begin
  for i := 3 to 3 do WriteLn('to ', i);
  for i := 3 downto 3 do WriteLn('downto ', i);
  for i := 0 to 0 do WriteLn('zero ', i);
  for i := -7 to -7 do WriteLn('neg ', i);
  for c := 'a' to 'a' do WriteLn('char ', c);
  for c := 'z' downto 'z' do WriteLn('char ', c)
end.
