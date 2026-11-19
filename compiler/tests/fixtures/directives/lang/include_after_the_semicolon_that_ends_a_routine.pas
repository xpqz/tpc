program IncludeAfterTheSemicolon;
procedure Q; begin WriteLn('q'); end;
{$I MORE.INC}
begin
  Q;
  R;
end.
