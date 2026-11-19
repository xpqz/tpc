program DirectivesInSkippedText;
{ only conditional directives count in skipped text; the others are not even checked }
{$IFDEF X}
{$Z} {$R} {$I+} {$I MISSING} {$R+,Q} {$DEFINE}
{$ENDIF}
begin
  WriteLn('ok');
end.
