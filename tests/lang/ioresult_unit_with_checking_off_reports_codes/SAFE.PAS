unit Safe;
interface
var
  LastCode: Integer;
function TryOpen(name: string): Boolean;
implementation
{$I-}
function TryOpen(name: string): Boolean;
var
  f: Text;
begin
  Assign(f, name);
  Reset(f);
  LastCode := IOResult;
  if LastCode = 0 then
  begin
    Close(f);
    LastCode := IOResult;
  end;
  TryOpen := LastCode = 0;
end;
begin
  LastCode := 0;
end.
