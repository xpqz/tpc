program IfdefSymbolNamesIgnoreCase;
{$DEFINE MixedCase}
begin
{$IFDEF MIXEDCASE}
  WriteLn('upper matches');
{$ENDIF}
{$IFDEF mixedcase}
  WriteLn('lower matches');
{$ENDIF}
{$ifdef MiXeDcAsE}
  WriteLn('mixed matches');
{$endif}
{$IFDEF MIXED}
  WriteLn('prefix must not match');
{$ENDIF}
end.
