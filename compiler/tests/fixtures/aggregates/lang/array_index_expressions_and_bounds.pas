program ArrayIndexExpressionsAndBounds;
const First = -3; Last = First + 6;
type Span = array[First..Last] of Integer; Tiny = array[5..5] of Byte;
     Wide = array[-100..100] of ShortInt;
var s: Span; t: Tiny; w: Wide; i: Integer;
function Twice(n: Integer): Integer;
begin
  Twice := n * 2
end;
begin
  for i := First to Last do s[i] := i * i;
  WriteLn(s[First], ' ', s[0], ' ', s[Last], ' ', s[Twice(1) - 3], ' ', s[s[1] - 2]);
  t[5] := 200;
  WriteLn(t[5], ' ', t[10 - 5]);
  w[-100] := -128; w[100] := 127; w[0] := w[-100] + w[100];
  WriteLn(w[-100], ' ', w[100], ' ', w[0], ' ', w[Twice(-50)]);
  i := 2;
  s[i + 1] := s[i - 1] + s[-i];
  WriteLn(s[3], ' ', SizeOf(Span), ' ', SizeOf(Tiny), ' ', SizeOf(Wide))
end.
