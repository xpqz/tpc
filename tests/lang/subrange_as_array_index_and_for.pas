program SubrangeIndex;
type Idx = 5..9;
var a: array[Idx] of Integer; i: Idx; s: Integer;
begin
  for i := 5 to 9 do a[i] := i * i;
  s := 0;
  for i := 5 to 9 do s := s + a[i];
  WriteLn(s, ' ', a[5], ' ', a[9]);
  for i := 9 downto 5 do Write(a[i], ' ');
  WriteLn
end.
