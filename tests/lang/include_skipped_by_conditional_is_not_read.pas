program IncludeSkippedByConditionalIsNotRead;
{ the included file does not exist; it is never opened }
{$IFDEF NEVER}
{$I MISSING.INC}
{$ENDIF}
begin
  WriteLn('ok');
end.
