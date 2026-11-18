program RepeatBasic;
var i: Integer;
begin
  i := 0;
  repeat
    i := i + 3;
    Write(i, ' ')
  until i >= 20;
  WriteLn;
  i := 10;
  repeat
    Write(i, ' ');
    i := i - 4
  until i < 0;
  WriteLn
end.
