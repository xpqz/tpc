program VarParamAliasLocalAndValue;
{ one variable passed by reference and by value in the same call: the value is a copy taken
  before the call, the reference sees every write; arguments are evaluated before the call }
var g: Integer;
function Next: Integer; begin g := g + 1; Next := g end;
procedure Both(var r: Integer; v: Integer);
begin
  r := r + 100;
  WriteLn(r, ' ', v)
end;
procedure Twice(var r: Integer; v: Integer; var s: Integer);
begin
  r := 1; s := s + v;
  WriteLn(r, ' ', v, ' ', s)
end;
procedure Local;
var x: Integer;
begin
  x := 5;
  Both(x, x);
  WriteLn(x);
  Twice(x, x, x);
  WriteLn(x)
end;
begin
  g := 0;
  Both(g, g);
  Both(g, Next);
  Twice(g, Next, g);
  WriteLn(g);
  Local
end.
