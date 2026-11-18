program SubrangeBasic;
type
  Digit = 0..9;
  Small = -5..5;
  Letter = 'a'..'z';
  Pct = 0..100;
var d: Digit; s: Small; l: Letter; p: Pct;
begin
  d := 7; s := -5; l := 'q'; p := 100;
  WriteLn(d, ' ', s, ' ', l, ' ', p);
  d := d + 2; s := s + 10;
  WriteLn(d, ' ', s);
  WriteLn(SizeOf(Digit), ' ', SizeOf(Small), ' ', SizeOf(Letter), ' ', SizeOf(Pct))
end.
