var l: LongInt;
begin
  l := $FFFFFFFF;
  case l of
    $FFFFFFFF: writeln('minus one');
  end;
  l := $80000000;
  writeln(l = $80000000, ' ', l < 0, ' ', l - 1);
  writeln($FFFFFFFF = -1, ' ', $FFFF = 65535, ' ', $FFFF > 0);
end.
