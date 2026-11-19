program PredefinedSymbols;
{ the symbols TP 5.0 defines, and a sample of those it does not }
begin
{$IFDEF VER50} WriteLn('VER50'); {$ENDIF}
{$IFDEF MSDOS} WriteLn('MSDOS'); {$ENDIF}
{$IFDEF CPU86} WriteLn('CPU86'); {$ENDIF}
{$IFDEF CPU87} WriteLn('CPU87'); {$ENDIF}
{$IFNDEF VER40} WriteLn('no VER40'); {$ENDIF}
{$IFNDEF VER55} WriteLn('no VER55'); {$ENDIF}
{$IFNDEF VER60} WriteLn('no VER60'); {$ENDIF}
{$IFNDEF CPU386} WriteLn('no CPU386'); {$ENDIF}
{$IFNDEF WINDOWS} WriteLn('no WINDOWS'); {$ENDIF}
{$IFNDEF DPMI} WriteLn('no DPMI'); {$ENDIF}
{$IFNDEF FPC} WriteLn('no FPC'); {$ENDIF}
{$IFDEF ver50} WriteLn('lower case matches'); {$ENDIF}
end.
