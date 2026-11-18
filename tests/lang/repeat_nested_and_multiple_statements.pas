program RepeatNestedAndMultipleStatements;
var i, j: Integer;
begin
  i := 0;
  repeat
    i := i + 1;
    j := 0;
    repeat
      j := j + 1;
      Write(i * 10 + j, ' ')
    until j = 3;
    WriteLn
  until i = 3
end.
