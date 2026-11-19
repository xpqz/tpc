var w: Word; b: Byte; s: ShortInt; l: LongInt; i: Integer; r: Real; c1: Word;
begin
  Val('65535', w, c1); writeln(w, ' ', c1);
  Val('65536', w, c1); writeln(w, ' ', c1);
  Val('-1', w, c1); writeln(w, ' ', c1);
  Val('300', b, c1); writeln(b, ' ', c1);
  Val('200', s, c1); writeln(s, ' ', c1);
  Val('99999', i, c1); writeln(i, ' ', c1);
  Val('123456', l, c1); writeln(l, ' ', c1);
  Val('abc', w, c1); writeln(w, ' ', c1);
  Val('$FF', w, c1); writeln(w, ' ', c1);
  Val('1.5', r, c1); writeln(r:0:1, ' ', c1);
  Val('xyz', r, c1); writeln(r:0:1, ' ', c1);
end.
