program FibonacciIterative;
var
  a, b, t: LongInt;
  i: Integer;
begin
  a := 0;
  b := 1;
  for i := 0 to 40 do
  begin
    WriteLn(i:2, ' ', a);
    t := a + b;
    a := b;
    b := t
  end
end.
