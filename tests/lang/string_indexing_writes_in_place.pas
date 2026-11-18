program StringInPlace;
var s: String[8]; i: Integer;
begin
  s := 'abcdefgh';
  for i := 1 to 8 do if Odd(i) then s[i] := Chr(Ord(s[i]) - 32);
  WriteLn(s);
  s := '';
  s[0] := Chr(5);
  for i := 1 to 5 do s[i] := Chr(48 + i);
  WriteLn(s, Length(s))
end.
