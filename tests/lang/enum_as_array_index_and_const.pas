program EnumArrayIndex;
type Color = (Red, Green, Blue);
const Fav = Blue;
var cnt: array[Color] of Integer; c: Color;
begin
  for c := Red to Blue do cnt[c] := (Ord(c) + 1) * 10;
  cnt[Fav] := cnt[Fav] + 1;
  for c := Red to Blue do Write(cnt[c], ' ');
  WriteLn;
  WriteLn(cnt[Succ(Red)], ' ', cnt[Pred(Blue)], ' ', cnt[Color(2)])
end.
