unit Base;
interface
var BaseCalls: Integer;
function Next: Integer;
implementation
function Next: Integer;
begin
  BaseCalls := BaseCalls + 1;
  Next := BaseCalls;
end;
begin
  BaseCalls := 0;
  WriteLn('init Base');
end.
