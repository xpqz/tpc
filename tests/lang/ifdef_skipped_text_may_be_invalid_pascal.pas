program IfdefSkippedTextMayBeInvalidPascal;
{ skipped text is not parsed and not checked, only scanned for directives }
begin
{$IFDEF NOPE}
  this is not pascal at all ;;; begin begin end end
  x := y +* z / undeclared(1, 2;
  procedure 99 const type
{$ENDIF}
  WriteLn('fine');
end.
