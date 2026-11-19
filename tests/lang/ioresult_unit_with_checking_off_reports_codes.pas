program IoresultCheckingSwitchIsLocalToAUnit;
{ a unit compiled with I/O checking off turns failures into return values }
uses Safe;
begin
  WriteLn(TryOpen('YES.TXT'));
  WriteLn(TryOpen('NO.TXT'));
  WriteLn(LastCode);
end.
