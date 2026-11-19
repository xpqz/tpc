program IfdefConditionalSpanningAnIncludeBoundary;
{ the IFDEF is opened and closed in the main file, with an include in between }
{$IFDEF NOPE}
{$I MISSING.INC}
{$ELSE}
{$I PRESENT.INC}
{$ENDIF}
begin
  WriteLn(Value);
end.
