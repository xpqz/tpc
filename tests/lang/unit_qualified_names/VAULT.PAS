unit Vault;
interface
const
  Limit = 99;
type
  TKey = Integer;
var
  Stored: Integer;
function Triple(n: Integer): Integer;
function High: Integer;
implementation
function Triple(n: Integer): Integer;
begin
  Triple := n * 3;
end;
function High: Integer;
begin
  High := Limit + 1;
end;
end.
