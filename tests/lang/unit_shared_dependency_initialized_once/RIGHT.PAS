unit Right;
interface
uses Base;
function RightValue: Integer;
implementation
function RightValue: Integer;
begin
  RightValue := Next * 10;
end;
begin
  WriteLn('init Right');
end.
