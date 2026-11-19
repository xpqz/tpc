program IncludeThatEndsInsideAStatementPart;
procedure Q;
{$I OPEN.INC}
  WriteLn('q');
end;
begin Q; end.
