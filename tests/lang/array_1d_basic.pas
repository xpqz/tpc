program Array1D;
var a: array[1..10] of Integer; i, s: Integer;
begin
  for i := 1 to 10 do a[i] := i * 3 - 7;
  s := 0;
  for i := 1 to 10 do s := s + a[i];
  WriteLn(s);
  for i := 10 downto 1 do Write(a[i], ' ');
  WriteLn;
  a[1] := a[10]; a[10] := 0;
  WriteLn(a[1], ' ', a[10], ' ', a[a[10] + 5])
end.
