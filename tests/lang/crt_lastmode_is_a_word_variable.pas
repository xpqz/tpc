uses Crt;
var w: Word;
begin
  w := LastMode;
  writeln(w, ' ', SizeOf(LastMode));
  LastMode := 7;
  writeln(LastMode);
  Inc(LastMode);
  writeln(LastMode);
end.
