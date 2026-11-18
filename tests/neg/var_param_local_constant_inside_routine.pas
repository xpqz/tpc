program NegTest;
procedure Set1(var x: Integer);
begin
  x := 1
end;
procedure Outer;
const Limit = 10;
begin
  Set1(Limit)
end;
begin
  Outer
end.
