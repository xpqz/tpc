unit Left;
interface
uses Base;
function LeftValue: Integer;
implementation
function LeftValue: Integer;
begin
  LeftValue := Next;
end;
begin
  WriteLn('init Left');
end.
