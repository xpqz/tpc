program SubrangeSignedStorage;
type S1 = 0..100; U1 = 0..200; W1 = 0..1000; B1 = False..True; Ch1 = 'a'..'z';
var s: S1; u: U1; w: W1; b: B1; ch: Ch1;
procedure Show(x: Integer);
begin
  WriteLn(x)
end;
begin
  s := 0; s := Pred(s); Show(Ord(s));
  u := 0; u := Pred(u); Show(Ord(u));
  w := 0; w := Pred(w); Show(Ord(w));
  b := False; b := Pred(b); Show(Ord(b));
  ch := 'a'; ch := Pred(ch); Show(Ord(ch))
end.
