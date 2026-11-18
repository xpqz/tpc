program ProcVarCallbackRecursion;
{$F+}
{ a procedural value that recurses through its own caller: depth 500 through Walk and
  the callback together }
type TVisit = procedure(n: Integer);
var visited: LongInt; depthMax: Integer;
procedure Walk(n: Integer; v: TVisit); forward;
procedure Visit(n: Integer);
begin
  visited := visited + n;
  if n > depthMax then depthMax := n;
  if n < 500 then Walk(n + 1, Visit)
end;
procedure Walk(n: Integer; v: TVisit);
begin
  v(n)
end;
begin
  visited := 0; depthMax := 0;
  Walk(1, Visit);
  WriteLn(visited, ' ', depthMax)
end.
