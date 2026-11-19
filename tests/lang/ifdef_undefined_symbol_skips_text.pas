program IfdefUndefinedSymbolSkipsText;
begin
{$IFDEF DEBUG}
  WriteLn('debug on');
{$ENDIF}
  WriteLn('done');
end.
