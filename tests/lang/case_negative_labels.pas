program CaseNegativeLabels;
var n: Integer;
begin
  for n := -6 to 6 do
  begin
    Write(n:3, ' ');
    case n of
      -5..-3: WriteLn('low');
      -2, -1: WriteLn('slightly negative');
      0: WriteLn('zero');
      1..2: WriteLn('slightly positive');
    else
      WriteLn('out of range')
    end
  end
end.
