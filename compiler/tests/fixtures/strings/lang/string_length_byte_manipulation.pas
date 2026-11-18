program StringLengthByteManipulation;
var s, t: String; n: Integer;
begin
  s := 'abc';
  s[0] := Chr(2);
  WriteLn(s, Length(s));
  s[0] := Chr(5);
  WriteLn(Length(s), s[3], s[4] = 'c');
  s[4] := 'X';
  WriteLn(s[4], Length(s));
  s := 'hello';
  s[0] := Chr(Length(s) - 1);
  WriteLn(s, '|', Length(s));
  t := s + 'o';
  WriteLn(t);
  s[0] := Chr(0);
  WriteLn('[', s, ']', Length(s) = 0, s = '');
  s := 'abc';
  n := Ord(s[0]);
  WriteLn(n, Ord(s[1]), Ord(s[n]));
  s[1] := Chr(Ord(s[1]) + 1);
  s[n] := Succ(s[n]);
  WriteLn(s);
  s[0] := Chr(255);
  WriteLn(Length(s), Ord(s[0]))
end.
