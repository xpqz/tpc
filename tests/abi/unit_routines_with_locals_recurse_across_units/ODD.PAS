unit Odd;
interface
function IsOdd(n: Integer): Boolean;
implementation
uses Even;
function IsOdd(n: Integer): Boolean;
var
  local: Integer;
begin
  local := n;
  Depth := Depth + 1;
  if n = 0 then IsOdd := False
  else IsOdd := IsEven(local - 1);
end;
end.
