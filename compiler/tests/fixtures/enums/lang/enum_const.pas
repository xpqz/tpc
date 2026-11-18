program EnumConst;
type Color = (Red, Green, Blue);
const Fav = Green; Other = Blue; Count = 3;
var c: Color;
begin
  c := Fav;
  WriteLn(Ord(c), Ord(Other), Count);
  case c of
    Red: WriteLn('red');
    Fav: WriteLn('fav')
  end;
  if c = Fav then WriteLn('is fav');
  for c := Red to Other do Write(Ord(c));
  WriteLn;
  c := Succ(Fav);
  WriteLn(c = Other)
end.
