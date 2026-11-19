program IncludeWithItsOwnConditional;
{$DEFINE BIG}
{$I PICK.INC}
begin
  WriteLn(Size);
{$IFDEF SMALLER}
  WriteLn('smaller is defined');
{$ELSE}
  WriteLn('smaller is not defined');
{$ENDIF}
end.
