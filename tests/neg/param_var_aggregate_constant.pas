program NegTest;
type A = array[1..3] of Integer;
procedure P(var a: A);
begin
end;
const C = 1;
begin
  P(C)
end.
