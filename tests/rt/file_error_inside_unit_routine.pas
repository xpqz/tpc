program FileErrorInsideUnitRoutine;
uses Loader;
begin
  WriteLn('start');
  WriteLn(FirstLine('GOOD.TXT'));
  WriteLn(FirstLine('BAD.TXT'));
  WriteLn('not reached');
end.
