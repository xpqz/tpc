var l: LongInt;
begin
  l := $FFFFFFFF;
  writeln($FFFFFFFF div 2, ' ', $8000 + $8000);
  writeln($FFFF + 1, ' ', $FFFF * $FFFF);
  writeln(l = $FFFFFFFF);
  writeln($80000001);
end.
