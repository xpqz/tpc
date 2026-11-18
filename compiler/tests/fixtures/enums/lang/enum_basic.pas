program EnumBasic;
type Color = (Red, Green, Blue);
var c, d: Color; e: (Left, Right);
begin
  c := Green; d := Blue; e := Right;
  WriteLn(Ord(c), Ord(d), Ord(e), Ord(Red), Ord(Left));
  if c = Green then WriteLn('eq');
  if c <> d then WriteLn('ne');
  if c < d then WriteLn('lt');
  if d > c then WriteLn('gt');
  if c <= Green then WriteLn('le');
  if d >= Blue then WriteLn('ge');
  if not (c > d) then WriteLn('not gt');
  c := d;
  if c = d then WriteLn('copied');
  WriteLn(c = d, ' ', c < d, ' ', Red < Blue, ' ', Blue < Red);
  c := Pred(d);
  WriteLn(Ord(c), Ord(Succ(c)), Ord(Pred(c)), Ord(Succ(Succ(c))));
  c := Succ(Red);
  WriteLn(Ord(c));
  e := Pred(Right);
  WriteLn(Ord(e), ' ', e = Left)
end.
