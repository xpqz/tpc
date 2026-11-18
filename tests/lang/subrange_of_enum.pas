program SubrangeOfEnum;
type
  Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
  Work = Mon..Fri;
  Weekend = Sat..Sun;
var w: Work; e: Weekend; d: Day;
begin
  w := Wed; e := Sun;
  d := w;
  WriteLn(Ord(w), ' ', Ord(e), ' ', Ord(d));
  for w := Mon to Fri do Write(Ord(w));
  WriteLn;
  for e := Sat to Sun do Write(Ord(e));
  WriteLn;
  d := e;
  WriteLn(Ord(d), ' ', w = Fri)
end.
