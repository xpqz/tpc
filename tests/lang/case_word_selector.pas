program CaseWordSelector;
var
  w: Word;
begin
  w := 40000;
  case w of
    0..32767: WriteLn('low half')
  else
    WriteLn('high half')
  end;
  w := 100;
  case w of
    0..32767: WriteLn('low half')
  else
    WriteLn('high half')
  end
end.
