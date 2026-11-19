const
  a = $80000000;
  b = $FFFFFFFF;
  c = $FFFFFFFE;
  d = $7FFFFFFF;
  e = $8000;
var
  l: LongInt;
  w: Word;
begin
  l := $80000000; writeln(l);
  writeln($FFFFFFFF, ' ', a, ' ', b, ' ', c, ' ', d);
  writeln(a - 1, ' ', a div 2, ' ', -a);
  l := $FFFFFFFF; writeln(l);
  writeln($FFFFFFFF + 1, ' ', $FFFF + 1, ' ', e + e);
  writeln(a shr 1, ' ', a and $FFFF, ' ', b xor 1);
  writeln(b < 0, ' ', a < 0);
end.
