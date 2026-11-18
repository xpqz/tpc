program VarParamGlobalAlsoAccessedDirectly;
{ a global passed as a var argument and also touched by name inside the callee }
var g: Integer;
procedure Both(var p: Integer);
begin
  p := 1;
  g := g + 10;
  WriteLn(p, ' ', g);
  p := p + 100;
  WriteLn(p, ' ', g)
end;
procedure Pass(var p: Integer);
begin
  Both(p);
  Both(g)
end;
begin
  g := 0;
  Both(g);
  Pass(g);
  WriteLn(g)
end.
