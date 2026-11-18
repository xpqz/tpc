program DivNested;
function Avg(sum, n: Integer): Integer;
begin
  Avg := sum div n;
end;
procedure Run(n: Integer);
begin
  Write('n=', n, ' avg=');
  WriteLn(Avg(100, n));
  if n > 0 then Run(n - 1);
end;
begin
  Run(3);
  WriteLn('not reached')
end.
