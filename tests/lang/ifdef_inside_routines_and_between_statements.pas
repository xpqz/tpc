program IfdefInsideRoutinesAndBetweenStatements;
{$DEFINE LOUD}
function Pick(n: Integer): Integer;
begin
{$IFDEF LOUD}
  Pick := n * 10;
{$ELSE}
  Pick := n;
{$ENDIF}
end;
var
  i: Integer;
begin
  for i := 1 to 3 do
  begin
    Write(Pick(i));
{$IFDEF LOUD}
    Write('!');
{$ENDIF}
    Write(' ');
  end;
  WriteLn;
end.
