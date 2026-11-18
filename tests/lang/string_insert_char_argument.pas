program StringInsertChar;
var s: String; c: Char;
begin
  s := 'ac'; c := 'b';
  Insert(c, s, 2);
  WriteLn(s);
  Insert(s[1], s, 4);
  WriteLn(s)
end.
