program ProcTypeStructuralCompatibility;
{$F+}
{ two procedural types with the same signature are assignment compatible (TP5 accepts it) }
type
  TA = procedure(x: Integer);
  TB = procedure(x: Integer);
var a: TA; b: TB;
procedure Show(x: Integer);
begin
  WriteLn('x=', x)
end;
begin
  a := Show;
  b := a;
  b(5);
  a(6)
end.
