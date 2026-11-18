program SubrangeBasic;
const N = 10;
type Small = 1..N; Neg = -5..5; Calc = 0..N*2+1; Letter = 'a'..'z'; Flag = False..True;
var s: Small; ng: Neg; c: Calc; l: Letter; f: Flag; i: Integer;
begin
  s := 7; ng := -3; c := 21; l := 'q'; f := True;
  i := s + ng + c;
  WriteLn(s, ' ', ng, ' ', c, ' ', l, ' ', f, ' ', i);
  s := i; WriteLn(s);
  l := Succ(l); WriteLn(l, Ord(l), Ord(Pred(l)));
  WriteLn(Ord(f), Succ(ng), Pred(s))
end.
