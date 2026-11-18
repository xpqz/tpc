program StringIndexing;
var s: String; i: Integer; c: Char;
begin
  s := 'Pascal';
  WriteLn(s[1], s[6], ' ', s[Length(s)]);
  for i := 1 to Length(s) do Write(s[i], '-');
  WriteLn;
  for i := Length(s) downto 1 do Write(s[i]);
  WriteLn;
  s[1] := 'p'; s[Length(s)] := 'L';
  WriteLn(s);
  c := s[2];
  s[2] := s[3]; s[3] := c;
  WriteLn(s);
  WriteLn(Ord(s[1]), ' ', s[1] = 'p', ' ', s[1] < 'q')
end.
