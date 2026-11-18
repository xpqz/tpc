program ComparisonsIntegers;
procedure Cmp(a, b: Integer);
begin
  WriteLn(a:6, b:6, ' ', a = b, ' ', a <> b, ' ', a < b, ' ', a > b, ' ', a <= b, ' ', a >= b)
end;
begin
  Cmp(1, 2);
  Cmp(2, 1);
  Cmp(3, 3);
  Cmp(-1, 1);
  Cmp(-32768, 32767);
  Cmp(-5, -6);
  Cmp(0, 0)
end.
