program UndefRemovesASymbol;
{$DEFINE A}
{$DEFINE A}
{$UNDEF NEVER_DEFINED}
{$UNDEF MSDOS}
begin
{$IFDEF A} WriteLn('A defined twice'); {$ENDIF}
{$IFDEF MSDOS} WriteLn('MSDOS still defined'); {$ELSE} WriteLn('MSDOS undefined'); {$ENDIF}
{$UNDEF A}
{$IFDEF A} WriteLn('A still defined'); {$ELSE} WriteLn('A undefined'); {$ENDIF}
{$DEFINE A}
{$IFDEF A} WriteLn('A defined again'); {$ENDIF}
{$IFDEF NEVER_DEFINED} WriteLn('never'); {$ELSE} WriteLn('never defined'); {$ENDIF}
end.
