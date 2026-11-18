program CaseRangesListsElse;
var n: Integer;
procedure Show(n: Integer);
begin
  Write(n:3, ': ');
  case n of
    0: WriteLn('zero');
    1, 3, 5, 7, 9: WriteLn('small odd');
    2, 4, 6, 8: WriteLn('small even');
    10..19: WriteLn('teens and ten');
    20..99, 100: WriteLn('medium');
  else
    WriteLn('other')
  end
end;
begin
  Show(0);
  Show(1);
  Show(4);
  Show(9);
  Show(10);
  Show(19);
  Show(20);
  Show(100);
  Show(101);
  Show(-1);
  Show(32767)
end.
