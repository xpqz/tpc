program EnumSuccPredBeyondEnds;
type Color = (Red, Green, Blue);
var c: Color;
begin
  c := Blue;
  WriteLn(Ord(Succ(c)), ' ', Ord(Pred(Red)));
  c := Succ(c);
  WriteLn(Ord(c));
  c := Red;
  c := Pred(c);
  WriteLn(Ord(c))
end.
