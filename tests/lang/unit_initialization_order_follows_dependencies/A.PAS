unit A;
interface
uses B;
function FromA: Integer;
implementation
function FromA: Integer;
begin
  FromA := FromB + 1;
end;
begin
  WriteLn('init A');
end.
