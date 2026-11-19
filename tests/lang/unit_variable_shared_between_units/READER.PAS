unit Reader;
interface
function Peek: Integer;
implementation
uses Store;
function Peek: Integer;
begin
  Peek := Cell;
end;
end.
