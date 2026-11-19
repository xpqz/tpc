unit Even;
interface
var Depth: Integer;
function IsEven(n: Integer): Boolean;
implementation
uses Odd;
function IsEven(n: Integer): Boolean;
var
  local: Integer;
begin
  local := n;
  Depth := Depth + 1;
  if n = 0 then IsEven := True
  else IsEven := IsOdd(local - 1);
end;
end.
