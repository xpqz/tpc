program IfdefConditionalAroundInclude;
{$DEFINE WITH_EXTRA}
{$IFDEF WITH_EXTRA}
{$I EXTRA.INC}
{$ELSE}
const
  Extra = 'no';
{$ENDIF}
begin
  WriteLn(Extra);
end.
