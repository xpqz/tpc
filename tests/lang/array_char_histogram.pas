program Histogram;
var h: array['a'..'z'] of Integer; s: String; i: Integer; c: Char;
begin
  for c := 'a' to 'z' do h[c] := 0;
  s := 'the quick brown fox jumps over the lazy dog';
  for i := 1 to Length(s) do
    if (s[i] >= 'a') and (s[i] <= 'z') then h[s[i]] := h[s[i]] + 1;
  for c := 'a' to 'z' do if h[c] > 1 then Write(c, h[c], ' ');
  WriteLn
end.
