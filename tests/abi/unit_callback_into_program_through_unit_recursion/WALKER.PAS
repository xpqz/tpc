unit Walker;
{$F+}
interface
type
  TVisit = procedure(depth: Integer);
procedure Descend(depth: Integer; visit: TVisit);
implementation
procedure Descend(depth: Integer; visit: TVisit);
begin
  visit(depth);
  if depth > 0 then
  begin
    Descend(depth - 1, visit);
    Descend(depth - 1, visit);
  end;
end;
end.
