program CharOps;
var c: Char; i: Integer;
begin
  c := 'M';
  WriteLn(Ord(c), ' ', Chr(Ord(c) + 1), ' ', Succ(c), ' ', Pred(c), ' ', Chr(0) < Chr(1));
  for c := 'a' to 'f' do Write(c);
  WriteLn;
  for i := 250 to 255 do Write(Ord(Chr(i)), ' ');
  WriteLn;
  WriteLn(UpCase('q'), UpCase('Q'), UpCase('5'), UpCase('z'))
end.
