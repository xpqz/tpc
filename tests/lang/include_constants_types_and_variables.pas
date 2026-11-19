program IncludeConstantsTypesAndVariables;
const
{$I CONSTS.INC}
type
{$I TYPES.INC}
var
{$I VARS.INC}
begin
  WriteLn(Size, ' ', Name);
  p.x := Size;
  p.y := Size * 2;
  WriteLn(p.x, ' ', p.y);
  counter := 5;
  counter := counter + 1;
  WriteLn(counter);
end.
