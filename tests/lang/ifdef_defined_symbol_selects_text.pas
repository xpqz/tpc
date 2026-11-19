program IfdefDefinedSymbolSelectsText;
{$DEFINE DEBUG}
begin
{$IFDEF DEBUG}
  WriteLn('debug on');
{$ENDIF}
  WriteLn('done');
end.
