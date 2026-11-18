program ForEmptyRange;
var
  i: Integer;
  c: Char;
  b: Boolean;
begin
  for i := 5 to 4 do WriteLn('never ', i);
  for i := 1 to 0 do WriteLn('never ', i);
  for i := 0 to -1 do WriteLn('never ', i);
  for i := 100 to -100 do WriteLn('never ', i);
  for c := 'b' to 'a' do WriteLn('never ', c);
  for b := True to False do WriteLn('never ', b);
  WriteLn('done')
end.
