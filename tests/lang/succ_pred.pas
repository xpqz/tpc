program SuccPred;
var
  c: Char;
  i: Integer;
  b: Boolean;
begin
  c := 'b';
  WriteLn(Succ(c), Pred(c));
  i := 10;
  WriteLn(Succ(i), ' ', Pred(i));
  i := -1;
  WriteLn(Succ(i), ' ', Pred(i));
  b := False;
  WriteLn(Succ(b), ' ', Pred(True))
end.
