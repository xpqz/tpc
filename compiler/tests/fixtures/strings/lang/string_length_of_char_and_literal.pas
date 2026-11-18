program StringLengthOfCharAndLiteral;
var c: Char; s: String[3];
begin
  c := 'a'; s := 'xyz';
  WriteLn(Length(c), Length('lit'), Length(s), Length('q'), Length(s[1]), Length(c) + Length(s));
  WriteLn(Pos(c, 'banana'), Pos('n', 'banana'), Pos(s[1], 'zyx'), Pos(c, s))
end.
