unit Hidden;
interface
function Reveal: Integer;
implementation
var
  Secret: Integer;
procedure Helper;
begin
  Secret := Secret + 100;
end;
function Reveal: Integer;
begin
  Secret := 7;
  Helper;
  Reveal := Secret;
end;
end.
