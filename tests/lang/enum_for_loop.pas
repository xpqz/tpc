program EnumForLoop;
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
var d: Day; n: Integer;
begin
  for d := Mon to Sun do Write(Ord(d), ' ');
  WriteLn;
  for d := Sun downto Mon do Write(Ord(d), ' ');
  WriteLn;
  n := 0;
  for d := Wed to Fri do n := n + Ord(d);
  WriteLn(n);
  n := 0;
  for d := Fri to Wed do n := n + 1;
  WriteLn(n);
  for d := Thu to Thu do WriteLn('once ', Ord(d));
  for d := Sun downto Sun do WriteLn('once ', Ord(d));
  for d := Mon to Mon do Write('a');
  WriteLn
end.
