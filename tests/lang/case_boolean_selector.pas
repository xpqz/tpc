program CaseBooleanSelector;
var i: Integer;
begin
  for i := 0 to 2 do
    case i > 0 of
      True: WriteLn(i, ' positive');
      False: WriteLn(i, ' not positive')
    end
end.
