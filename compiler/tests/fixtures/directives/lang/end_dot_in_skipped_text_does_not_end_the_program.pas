program EndDotInSkippedText;
{$IFDEF X}
begin WriteLn('x'); end.
{$ELSE}
begin WriteLn('y'); end.
{$ENDIF}
this text follows the final end and is never read ? 'unterminated
