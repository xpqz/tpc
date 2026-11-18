program StringResultN;
type S4 = String[4];
function Cut(s: String): S4; begin Cut := s end;
function Mix(a: S4; b: String): String; begin Mix := a + b end;
var s: String;
begin
  s := Mix(Cut('abcdefg'), Cut('hijklmn'));
  WriteLn(s, ' ', Length(s));
  s := Cut(Mix('12345', '678'));
  WriteLn(s)
end.
