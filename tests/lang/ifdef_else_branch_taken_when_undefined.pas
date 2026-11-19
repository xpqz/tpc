program IfdefElseBranchTakenWhenUndefined;
begin
{$IFDEF FAST}
  WriteLn('fast');
{$ELSE}
  WriteLn('slow');
{$ENDIF}
end.
