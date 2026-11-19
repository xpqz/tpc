unit Outer;
interface
const C = 2;
function F: Integer;
implementation
function F: Integer;
begin
  F := Outer.C * 10;
end;
end.
