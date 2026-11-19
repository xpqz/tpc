program IfdefSkippedTextIgnoresOtherDirectives;
{ a $DEFINE inside skipped text has no effect }
begin
{$IFDEF NOPE}
{$DEFINE LATER}
{$I MISSING.INC}
{$ENDIF}
{$IFDEF LATER}
  WriteLn('later is defined');
{$ELSE}
  WriteLn('later is not defined');
{$ENDIF}
end.
