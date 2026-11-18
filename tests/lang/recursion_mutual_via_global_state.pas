program RecursionMutualViaGlobalState;
var calls: Integer;
function CountDown(n: Integer): Integer;
begin
  calls := calls + 1;
  if n <= 0 then CountDown := calls
  else CountDown := CountDown(n - 2)
end;
begin
  calls := 0;
  WriteLn(CountDown(10));
  calls := 0;
  WriteLn(CountDown(9));
  calls := 0;
  WriteLn(CountDown(0))
end.
