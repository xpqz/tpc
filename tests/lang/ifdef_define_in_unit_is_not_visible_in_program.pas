program IfdefDefineInUnitIsNotVisibleInProgram;
{ a symbol defined in a unit's source stays local to that source file }
uses Flags;
begin
{$IFDEF FROM_UNIT}
  WriteLn('visible');
{$ELSE}
  WriteLn('not visible');
{$ENDIF}
  WriteLn(InUnit);
end.
