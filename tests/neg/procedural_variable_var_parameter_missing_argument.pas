program NegTest;
{$F+}
type T = procedure(var x: Integer; y: Integer);
var p: T; n: Integer;
procedure Q(var x: Integer; y: Integer);
begin
end;
begin
  p := Q;
  p(n)
end.
