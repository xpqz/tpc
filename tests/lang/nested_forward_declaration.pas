program NestedForwardDeclaration;
{ forward declarations inside a routine: mutual recursion between nested routines }
function Run(n: Integer): Integer;
var steps: Integer;
  function IsOdd(k: Integer): Boolean; forward;
  function IsEven(k: Integer): Boolean;
  begin
    steps := steps + 1;
    if k = 0 then IsEven := True else IsEven := IsOdd(k - 1)
  end;
  function IsOdd(k: Integer): Boolean;
  begin
    steps := steps + 1;
    if k = 0 then IsOdd := False else IsOdd := IsEven(k - 1)
  end;
begin
  steps := 0;
  if IsEven(n) then Run := steps else Run := -steps
end;
begin
  WriteLn(Run(0), ' ', Run(7), ' ', Run(10))
end.
