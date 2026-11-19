program IncludeGuarded;
{$I TYPES.INC}
{$I TYPES.INC}
var
  p: TPoint;
begin
  p.x := 1;
  WriteLn(p.x);
{$IFDEF TYPES_INC}
  WriteLn('guard defined');
{$ENDIF}
end.
