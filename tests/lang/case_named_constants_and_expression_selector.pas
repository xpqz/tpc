program CaseNamedConstantsAndExpressionSelector;
const
  Low = 10;
  High = Low + 10;
var n: Integer;
begin
  for n := 8 to 22 do
  begin
    case n mod 7 of
      0: Write('m0 ');
      1..2: Write('m12 ');
    else
      Write('m3+ ')
    end;
    case n of
      Low: WriteLn('Low');
      Low + 1..High - 1: WriteLn('between');
      High: WriteLn('High');
    else
      WriteLn('outside')
    end
  end
end.
