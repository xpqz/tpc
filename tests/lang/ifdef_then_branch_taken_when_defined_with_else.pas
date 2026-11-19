program IfdefThenBranchTakenWhenDefinedWithElse;
{$DEFINE FAST}
begin
{$IFDEF FAST}
  WriteLn('fast');
{$ELSE}
  WriteLn('slow');
{$ENDIF}
end.
