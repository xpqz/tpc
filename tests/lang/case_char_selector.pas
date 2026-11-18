program CaseCharSelector;
var c: Char;
procedure Kind(c: Char);
begin
  case c of
    'a', 'e', 'i', 'o', 'u': Write('vowel');
    'b'..'d', 'f'..'h', 'j'..'n', 'p'..'t', 'v'..'z': Write('consonant');
    'A'..'Z': Write('upper');
    '0'..'9': Write('digit');
    ' ': Write('space');
  else
    Write('other')
  end;
  WriteLn(' ', c)
end;
begin
  Kind('a');
  Kind('b');
  Kind('z');
  Kind('M');
  Kind('7');
  Kind(' ');
  Kind('!');
  Kind('{')
end.
