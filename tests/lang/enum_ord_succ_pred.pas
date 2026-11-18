program EnumOrdSuccPred;
type Color = (Red, Green, Blue, Yellow);
var c: Color; i: Integer;
begin
  c := Green;
  WriteLn(Ord(Red), ' ', Ord(c), ' ', Ord(Yellow));
  WriteLn(Ord(Succ(Red)), ' ', Ord(Succ(c)), ' ', Ord(Pred(Yellow)), ' ', Ord(Pred(Blue)));
  c := Succ(Succ(c));
  WriteLn(Ord(c));
  c := Pred(c);
  WriteLn(Ord(c));
  for i := 0 to 3 do begin
    c := Color(i);
    Write(Ord(c), ' ')
  end;
  WriteLn
end.
