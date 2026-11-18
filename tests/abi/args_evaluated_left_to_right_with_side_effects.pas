program ArgsEvaluatedLeftToRightWithSideEffects;
{ arguments are evaluated before the call, left to right; function calls in arguments
  change a global used by later arguments }
var g: Integer;
function Next: Integer; begin g := g + 1; Next := g end;
procedure Show(a, b, c: Integer);
begin
  WriteLn(a, ' ', b, ' ', c, ' ', g)
end;
procedure ShowVar(var v: Integer; a: Integer);
begin
  v := v + 100;
  WriteLn(v, ' ', a, ' ', g)
end;
begin
  g := 0;
  Show(Next, Next, Next);
  Show(g, Next, g);
  ShowVar(g, Next);
  ShowVar(g, g)
end.
