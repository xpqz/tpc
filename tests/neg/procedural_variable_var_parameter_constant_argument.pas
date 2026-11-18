program NegTest;
{$F+}
type T = procedure(var x: Integer);
var p: T;
procedure Q(var x: Integer);
begin
end;
begin
  p := Q;
  p(5)
end.
