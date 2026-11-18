program SubrangeMixedWithWord;
type S1 = 0..100; U1 = 0..200; Color = (Red, Green, Blue);
var s: S1; u: U1; c: Color; w: Word; b: Byte; sh: ShortInt;
begin
  s := 1; u := 1; c := Green; w := 65535; b := 1; sh := 1;
  WriteLn(s + w, ' ', u + w, ' ', Ord(c) + w, ' ', b + w, ' ', sh + w);
  WriteLn(-s, ' ', s shl 9, ' ', u shl 9, ' ', not s, ' ', not u)
end.
