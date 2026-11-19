program IncludeUndefines;
{$DEFINE FLAG}
{$I CLEAR.INC}
begin
{$IFDEF FLAG}
  WriteLn('flag is defined');
{$ELSE}
  WriteLn('flag was undefined by the include');
{$ENDIF}
end.
