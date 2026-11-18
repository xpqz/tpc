program PrimesByFunction;
var n, count: Integer;
function IsPrime(n: Integer): Boolean;
var d: Integer;
begin
  IsPrime := n >= 2;
  d := 2;
  while d * d <= n do
  begin
    if n mod d = 0 then IsPrime := False;
    d := d + 1
  end
end;
begin
  count := 0;
  for n := 1 to 1000 do
    if IsPrime(n) then count := count + 1;
  WriteLn(count);
  WriteLn(IsPrime(1), IsPrime(2), IsPrime(97), IsPrime(100), IsPrime(7919))
end.
