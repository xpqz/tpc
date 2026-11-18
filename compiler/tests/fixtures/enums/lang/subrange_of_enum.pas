program SubrangeOfEnum;
type Color = (Red, Green, Blue, Cyan, Pink); Warm = Red..Green; Cool = Blue..Pink;
var w: Warm; c: Color; k: Cool;
begin
  w := Green; c := w; k := Cyan;
  if c < k then WriteLn('lt');
  c := k; w := c;
  WriteLn(Ord(w), Ord(c));
  for k := Blue to Pink do Write(Ord(k));
  WriteLn
end.
