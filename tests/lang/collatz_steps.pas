program CollatzSteps;
var
  n, start, steps, peak: LongInt;
begin
  for start := 1 to 30 do
  begin
    n := start;
    steps := 0;
    peak := n;
    while n <> 1 do
    begin
      if Odd(n) then n := 3 * n + 1 else n := n div 2;
      if n > peak then peak := n;
      steps := steps + 1
    end;
    WriteLn(start:3, steps:5, peak:7)
  end
end.
