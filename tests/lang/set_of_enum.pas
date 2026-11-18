program SetEnum;
type
  Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
  DaySet = set of Day;
var work, weekend, all, s: DaySet; d: Day;
procedure Show(s: DaySet);
var d: Day;
begin
  for d := Mon to Sun do
    if d in s then Write(Ord(d), ' ');
  WriteLn;
end;
begin
  work := [Mon..Fri];
  weekend := [Sat, Sun];
  all := work + weekend;
  Show(work);
  Show(weekend);
  Show(all);
  Show(work * [Wed..Sat]);
  Show(all - [Tue, Thu]);
  WriteLn(work * weekend = [], ' ', all >= work, ' ', weekend <= work, ' ', Sat in weekend);
  s := [];
  for d := Mon to Sun do
    if Odd(Ord(d)) then s := s + [d];
  Show(s);
  d := Wed;
  WriteLn(d in work, ' ', Succ(d) in [Thu], ' ', Pred(d) in weekend)
end.
