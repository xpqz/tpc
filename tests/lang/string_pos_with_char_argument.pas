program StringPosChar;
var s: String; c: Char;
begin
  s := 'hello world';
  c := 'o';
  WriteLn(Pos(c, s), ' ', Pos(' ', s), ' ', Pos(s[1], s), ' ', Pos(s[2] + s[3], s))
end.
