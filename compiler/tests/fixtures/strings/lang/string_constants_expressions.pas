program StringConstantsExpressions;
const First = 'abc'; Joined = First + 'def'; Pair = 'x' + 'y'; Blank = ''; Letter = 'q';
var s: String; t: String[4]; c: Char;
begin
  WriteLn(Joined, Pair, Length(Joined), Length(Blank), Length(First));
  s := Joined; t := Joined; c := Letter;
  WriteLn(s, ' ', t, ' ', c);
  WriteLn(First = 'abc', Joined > First, Blank = '', Letter = 'q', Letter + Letter);
  s := Letter + First + Letter;
  WriteLn(s, Pos(Letter, s), Pos(First, Joined));
  if s[1] = Letter then WriteLn('starts with q')
end.
