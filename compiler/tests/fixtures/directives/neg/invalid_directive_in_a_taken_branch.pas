program InvalidDirectiveInATakenBranch;
{$IFDEF X}
{$ENDIF}
{$IFNDEF X} {$Z} {$ENDIF}
begin end.
