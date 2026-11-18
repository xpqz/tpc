program EnumBoundaryLoops;
type Color = (Red, Green, Blue); Big = 250..255; Whole = 0..255; Neg = -128..127;
var c: Color; b: Big; w: Whole; n: Neg; sum: Integer;
begin
  for c := Red to Blue do Write(Ord(c));
  for c := Blue downto Red do Write(Ord(c));
  WriteLn;
  sum := 0;
  for b := 250 to 255 do sum := sum + b;
  WriteLn(sum);
  for b := 255 downto 250 do sum := sum - 1;
  WriteLn(sum);
  sum := 0;
  for w := 253 to 255 do sum := sum + 1;
  for w := 2 downto 0 do sum := sum + 10;
  WriteLn(sum);
  sum := 0;
  for n := 125 to 127 do sum := sum + n;
  for n := -126 downto -128 do sum := sum + n;
  WriteLn(sum)
end.
