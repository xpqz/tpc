program IfdefDefineInProgramIsNotVisibleInUnit;
{$DEFINE FROM_PROGRAM}
uses Probe;
begin
{$IFDEF FROM_PROGRAM}
  WriteLn('program sees it');
{$ENDIF}
  WriteLn(UnitSees);
end.
