program SubrangeConsts;
const Lo = 3; Hi = Lo * 4;
type RT = Lo..Hi; NT = -Hi..-Lo;
var r: RT; n: NT;
begin
  r := Lo; n := -Hi;
  WriteLn(r, ' ', n);
  r := Hi; n := -Lo;
  WriteLn(r, ' ', n);
  WriteLn(SizeOf(RT), ' ', SizeOf(NT))
end.
