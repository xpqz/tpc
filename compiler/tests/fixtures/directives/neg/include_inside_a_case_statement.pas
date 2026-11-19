program IncludeInsideACaseStatement;
var n: Integer;
begin
  n := 1;
  case n of
    1: WriteLn('one');
{$I STMT.INC}
  end;
end.
