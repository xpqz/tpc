program ArrayEnumCharBooleanIndexes;
type Color = (Red, Green, Blue, Cyan); Warm = Red..Green;
var counts: array[Color] of Integer;
    seen: array['a'..'e'] of Boolean;
    flip: array[Boolean] of Char;
    hot: array[Warm] of ShortInt;
    grid: array[Color, Boolean] of Word;
    c: Color; ch: Char; b: Boolean;
begin
  for c := Red to Cyan do counts[c] := (Ord(c) + 1) * 100;
  WriteLn(counts[Red], ' ', counts[Green], ' ', counts[Blue], ' ', counts[Cyan]);
  for ch := 'a' to 'e' do seen[ch] := Odd(Ord(ch));
  for ch := 'a' to 'e' do Write(seen[ch], ' ');
  WriteLn;
  flip[False] := 'N'; flip[True] := 'Y';
  WriteLn(flip[False], flip[True], flip[3 > 2], flip[Red > Blue]);
  hot[Red] := -100; hot[Green] := 100;
  WriteLn(hot[Red], ' ', hot[Green], ' ', hot[Red] + hot[Green]);
  for c := Red to Cyan do
    for b := False to True do
      grid[c, b] := Ord(c) * 10000 + Ord(b) * 5000;
  WriteLn(grid[Cyan, True], ' ', grid[Blue][False], ' ', grid[Red, True], ' ', grid[Green][True]);
  counts[Succ(Red)] := counts[Pred(Cyan)] + 1;
  WriteLn(counts[Green])
end.
