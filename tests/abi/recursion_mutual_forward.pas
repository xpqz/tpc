program RecursionMutualForward;
{ mutual recursion through forward declarations, depth 1000 }
function IsOdd(n: Integer): Boolean; forward;
function IsEven(n: Integer): Boolean;
begin
  if n = 0 then IsEven := True else IsEven := IsOdd(n - 1)
end;
function IsOdd(n: Integer): Boolean;
begin
  if n = 0 then IsOdd := False else IsOdd := IsEven(n - 1)
end;
var i: Integer;
begin
  for i := 0 to 5 do WriteLn(i, ' ', IsEven(i), ' ', IsOdd(i));
  WriteLn(IsEven(1000), ' ', IsOdd(999), ' ', IsEven(777))
end.
