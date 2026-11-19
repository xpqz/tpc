unit Late;
interface
implementation
var
  zero: Integer;
begin
  WriteLn('late initialization');
  zero := 0;
  WriteLn(10 div zero);
end.
