program IfCompoundAndNested;
var i, evens, odds: Integer;
begin
  evens := 0;
  odds := 0;
  for i := 1 to 10 do
    if i mod 2 = 0 then
    begin
      evens := evens + i;
      if i > 6 then
        WriteLn('big even ', i)
    end
    else
    begin
      odds := odds + i;
      if i < 4 then WriteLn('small odd ', i)
    end;
  WriteLn(evens, ' ', odds);
  if evens > odds then ;
  if True then begin end else WriteLn('never')
end.
