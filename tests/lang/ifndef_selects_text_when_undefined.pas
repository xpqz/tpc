program IfndefSelectsTextWhenUndefined;
begin
{$IFNDEF TRACE}
  WriteLn('quiet');
{$ELSE}
  WriteLn('trace');
{$ENDIF}
{$DEFINE TRACE}
{$IFNDEF TRACE}
  WriteLn('quiet again');
{$ELSE}
  WriteLn('trace now');
{$ENDIF}
end.
