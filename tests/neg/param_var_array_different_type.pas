program NegTest;
type A = array[1..3] of Integer;
procedure P(var a: A);
begin
end;
var b: array[1..3] of Integer;
begin
  P(b)
end.
