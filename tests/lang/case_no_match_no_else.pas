program CaseNoMatchNoElse;
var n: Integer;
begin
  for n := 0 to 5 do
  begin
    case n of
      1: Write('one ');
      3..4: Write('three-four ')
    end;
    Write('.')
  end;
  WriteLn;
  case 99 of
    1: WriteLn('never')
  end;
  WriteLn('after')
end.
