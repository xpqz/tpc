program PerfectNumbers;
var n, d, sum: Integer;
begin
  for n := 2 to 500 do
  begin
    sum := 1;
    for d := 2 to n div 2 do
      if n mod d = 0 then sum := sum + d;
    if sum = n then WriteLn(n, ' is perfect')
  end
end.
