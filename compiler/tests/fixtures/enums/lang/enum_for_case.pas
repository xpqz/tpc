program EnumForCase;
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
var d: Day; n: Integer;
procedure Name(x: Day);
begin
  case x of
    Mon: Write('Mon');
    Tue, Wed: Write('Tue/Wed');
    Thu..Fri: Write('Thu-Fri');
    Sat: Write('Sat')
  else
    Write('other')
  end
end;
begin
  for d := Mon to Sun do
  begin
    Name(d); Write(' ')
  end;
  WriteLn;
  for d := Sun downto Mon do Write(Ord(d));
  WriteLn;
  n := 0;
  for d := Wed to Wed do n := n + 1;
  for d := Fri to Tue do n := n + 100;
  WriteLn(n);
  for d := Tue to Thu do
    for n := 1 to Ord(d) do Write(n);
  WriteLn;
  d := Sat;
  case d of
    Mon..Fri: WriteLn('week');
    Sat..Sun: WriteLn('weekend')
  end
end.
