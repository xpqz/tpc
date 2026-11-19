unit Boot;
interface
var
  Ready: Boolean;
  Value: Integer;
implementation
begin
  WriteLn('boot initialization');
  Ready := True;
  Value := 42;
end.
