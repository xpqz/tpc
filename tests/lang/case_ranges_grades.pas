program CaseRangesGrades;
var score: Integer;
begin
  score := 0;
  while score <= 100 do
  begin
    Write(score:3, ' ');
    case score of
      90..100: WriteLn('A');
      80..89: WriteLn('B');
      70..79: WriteLn('C');
      60..69: WriteLn('D');
      0..59: WriteLn('F')
    end;
    score := score + 15
  end;
  case 101 of
    90..100: WriteLn('A')
  else
    WriteLn('invalid')
  end
end.
