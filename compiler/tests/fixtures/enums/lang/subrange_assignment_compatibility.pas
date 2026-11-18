program SubrangeAssignmentCompatibility;
type Tn = 1..10; U = 5..8;
var a: Tn; u2: U; i: Integer; b: Byte; w: Word; l: LongInt;
begin
  a := 3; u2 := a; i := u2; b := u2; w := a; l := u2; a := b; a := l;
  u2 := i;
  WriteLn(u2, i, b, w, l)
end.
