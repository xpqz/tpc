program ForBoundsEvaluatedOnce;
var i, n, count: Integer;
begin
  n := 5;
  count := 0;
  for i := 1 to n do
  begin
    n := n + 10;
    count := count + 1
  end;
  WriteLn(count, ' ', n);
  n := 3;
  count := 0;
  for i := n downto 1 do
  begin
    n := 0;
    count := count + 1
  end;
  WriteLn(count, ' ', n);
  n := 2;
  for i := n to n * 3 do Write(i, ' ');
  WriteLn
end.
