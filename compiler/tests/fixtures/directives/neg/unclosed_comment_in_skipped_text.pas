program UnclosedCommentInSkippedText;
begin
{$IFDEF X}
  { never closed
{$ENDIF}
  WriteLn('a');
end.
