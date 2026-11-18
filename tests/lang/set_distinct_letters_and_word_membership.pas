program SetLetters;
var seen: set of Char; s: String; i, n: Integer; c: Char; t: String;
begin
  s := 'the quick brown fox jumps over the lazy dog';
  seen := [];
  for i := 1 to Length(s) do
    if s[i] <> ' ' then seen := seen + [s[i]];
  n := 0;
  t := '';
  for c := 'a' to 'z' do
    if c in seen then begin
      n := n + 1;
      t := t + c;
    end;
  WriteLn(n, ' ', t);
  seen := [];
  s := 'hello world';
  for i := 1 to Length(s) do seen := seen + [s[i]];
  n := 0;
  for c := #0 to #255 do
    if c in seen then n := n + 1;
  WriteLn(n)
end.
