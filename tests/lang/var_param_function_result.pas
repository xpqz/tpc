program VarParamFunctionResult;
{ a function with a var parameter returns a value and changes the caller's variable;
  the function is called in expressions }
var calls: Integer;
function Take(var pool: Integer; n: Integer): Integer;
begin
  calls := calls + 1;
  if n > pool then n := pool;
  pool := pool - n;
  Take := n
end;
var stock, got: Integer;
begin
  stock := 10;
  got := Take(stock, 4) + Take(stock, 4) + Take(stock, 4);
  WriteLn(got, ' ', stock, ' ', calls);
  if Take(stock, 1) = 0 then WriteLn('empty');
  stock := 9;
  WriteLn(Take(stock, 5):3, stock:3)
end.
