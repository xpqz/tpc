program IfdefSelectsDeclarations;
{$DEFINE BIG}
const
{$IFDEF BIG}
  Size = 1000;
{$ELSE}
  Size = 10;
{$ENDIF}
type
{$IFDEF SMALL}
  TNum = Byte;
{$ELSE}
  TNum = Integer;
{$ENDIF}
var
  n: TNum;
begin
  n := Size;
  WriteLn(n);
end.
