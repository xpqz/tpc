program SubrangeOfCharAndBoolean;
type Lower = 'a'..'z'; Digit = '0'..'9'; Truth = False..True; Yes = True..True;
var l: Lower; d: Digit; t: Truth; y: Yes; i: Integer;
begin
  l := 'm'; d := '7'; t := True; y := True;
  WriteLn(l, d, t, y);
  WriteLn(Ord(l), ' ', Ord(d), ' ', Ord(t), ' ', Ord(y));
  l := Succ(l); d := Pred(d);
  WriteLn(l, d, Ord(Succ(l)), Ord(Pred(d)));
  if l > 'm' then WriteLn('after m');
  if (d >= '0') and (d <= '9') then WriteLn('digit');
  i := Ord(d) - Ord('0');
  WriteLn(i);
  for l := 'x' to 'z' do Write(l);
  for d := '9' downto '7' do Write(d);
  WriteLn;
  t := not t;
  WriteLn(t, ' ', t or y, ' ', t and y)
end.
