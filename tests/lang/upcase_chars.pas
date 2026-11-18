program UpCaseTest;
var c: Char; s, t: String; i: Integer;
begin
  WriteLn(UpCase('a'), UpCase('m'), UpCase('z'), UpCase('A'), UpCase('Z'));
  WriteLn(UpCase('0'), UpCase(' '), UpCase('@'), UpCase('['), UpCase('`'), UpCase('{'));
  s := 'Hello, World 123';
  t := '';
  for i := 1 to Length(s) do t := t + UpCase(s[i]);
  WriteLn(t);
  c := 'q';
  WriteLn(UpCase(c) = 'Q', ' ', Ord(UpCase('b')), ' ', Ord(UpCase(#200)), ' ', Ord(UpCase(#0)));
  for c := 'x' to 'z' do Write(UpCase(c));
  WriteLn
end.
