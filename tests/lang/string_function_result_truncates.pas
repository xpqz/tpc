program StringFunctionTruncate;
type S4 = String[4];
function Short: S4; begin Short := 'abcdefgh' end;
type S6 = String[6];
function Cat(a, b: String): S6; begin Cat := a + b end;
function Id(s: String): String; begin Id := s end;
var s: String[3]; t: String;
begin
  WriteLn(Short, Length(Short));
  s := Short;
  WriteLn(s);
  t := Cat('1234', '5678');
  WriteLn(t, ' ', Length(t));
  s := Id('wxyz');
  WriteLn(s)
end.
