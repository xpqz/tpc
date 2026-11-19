program IfdefDefineInIncludeContinuesAfterInclude;
{$I SETFLAG.INC}
begin
{$IFDEF FLAGGED}
  WriteLn('flag set by include');
{$ELSE}
  WriteLn('flag not set');
{$ENDIF}
end.
