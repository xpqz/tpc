unit Vault;
interface
const
  Limit = 3;
type
  TKey = Integer;
  TRec = record x: Integer end;
var
  Total: Integer;
procedure Bump(n: Integer);
implementation
procedure Bump(n: Integer);
begin
  Total := Total + n;
end;
begin
  Total := 0;
end.
