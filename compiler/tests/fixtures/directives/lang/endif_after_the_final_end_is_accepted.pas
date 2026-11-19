program EndifAfterTheEnd;
{$IFNDEF X}
begin WriteLn('x'); end.
{$ENDIF}
