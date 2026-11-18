program CaseElseOptionalSemicolon;
var n: Integer;
begin
  for n := 1 to 3 do
  begin
    case n of
      1: WriteLn('one')
    else WriteLn('not one')
    end;
    case n of
      2: WriteLn('two');
    else
      WriteLn('not two');
    end
  end
end.
