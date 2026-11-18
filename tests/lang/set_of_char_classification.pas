program SetChars;
type CharSet = set of Char;
var s: String; i, digits, upper, lower, other, vowels: Integer;
    letters, alnum: CharSet;
begin
  letters := ['a'..'z', 'A'..'Z'];
  alnum := letters + ['0'..'9'];
  s := 'Hello, World 2024! TP5 rules_ok.';
  digits := 0; upper := 0; lower := 0; other := 0; vowels := 0;
  for i := 1 to Length(s) do begin
    if s[i] in ['0'..'9'] then digits := digits + 1
    else if s[i] in ['A'..'Z'] then upper := upper + 1
    else if s[i] in ['a'..'z'] then lower := lower + 1
    else other := other + 1;
    if UpCase(s[i]) in ['A', 'E', 'I', 'O', 'U'] then vowels := vowels + 1;
  end;
  WriteLn(digits, ' ', upper, ' ', lower, ' ', other, ' ', vowels);
  digits := 0;
  for i := 1 to Length(s) do
    if s[i] in alnum then digits := digits + 1;
  WriteLn(digits);
  digits := 0;
  for i := 1 to Length(s) do
    if not (s[i] in letters) then digits := digits + 1;
  WriteLn(digits)
end.
