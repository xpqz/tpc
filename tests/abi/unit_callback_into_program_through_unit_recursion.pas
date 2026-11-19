program UnitCallbackIntoProgramThroughUnitRecursion;
{$F+}
uses Walker;
var
  visited: Integer;
  trail: string;
procedure Visit(depth: Integer);
begin
  visited := visited + 1;
  trail := trail + Chr(Ord('0') + depth);
end;
begin
  visited := 0;
  trail := '';
  Descend(3, Visit);
  WriteLn(visited, ' ', trail);
end.
