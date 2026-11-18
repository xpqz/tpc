program RepeatRunsOnce;
var n: Integer;
begin
  n := 0;
  repeat
    n := n + 1;
    WriteLn('body ', n)
  until True;
  repeat
    WriteLn('once more')
  until n > 0;
  repeat until True;
  WriteLn(n)
end.
