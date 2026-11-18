program IfBooleanVariable;
var f, g: Boolean;
begin
  f := True;
  g := False;
  if f then WriteLn('f');
  if g then WriteLn('g');
  if not g then WriteLn('not g');
  if f and not g then WriteLn('f and not g');
  if f = g then WriteLn('equal') else WriteLn('different')
end.
