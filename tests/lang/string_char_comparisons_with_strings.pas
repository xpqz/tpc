program StringCharCompare;
var s: String; c: Char;
begin
  s := 'x'; c := 'x';
  WriteLn(s = c, ' ', c = s, ' ', s < 'y', ' ', 'w' < s, ' ', s = 'x');
  s := 'xy';
  WriteLn(s = c, ' ', s > c, ' ', c < s);
  s := '';
  WriteLn(s < c, ' ', c > s)
end.
