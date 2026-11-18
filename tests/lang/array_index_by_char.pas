program ArrayCharIndex;
var cnt: array[Char] of Byte; c: Char; w: array['a'..'e'] of Integer; n: Integer;
begin
  for c := #0 to #255 do cnt[c] := 0;
  cnt['x'] := 5; cnt[#0] := 1; cnt[#255] := 2;
  n := 0;
  for c := #0 to #255 do n := n + cnt[c];
  WriteLn(n, ' ', cnt['x'], ' ', cnt[#255], ' ', SizeOf(cnt));
  for c := 'a' to 'e' do w[c] := Ord(c) - 96;
  for c := 'e' downto 'a' do Write(w[c], ' ');
  WriteLn
end.
