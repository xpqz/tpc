program StringWriteForms;
var s, e: String; c: Char; n: Integer;
begin
  s := 'abc'; e := ''; c := 'z'; n := 5;
  WriteLn(s:6, '|', s:3, '|', s:2, '|', s:0, '|', s:1, '|');
  WriteLn(e:3, '|', e, '|', c:4, '|', c:1, '|', 'lit':5, '|', 'x':n, '|');
  WriteLn(s + s:8, '|', Copy(s, 2, 1):n + 1, '|', s[2]:3, '|');
  Write(s, e, c);
  Write(s:4);
  WriteLn;
  WriteLn(n:4, s:4, True:6, c:3);
  WriteLn(Length(s):3, ' ', Ord(c):5);
  Write('no newline');
  WriteLn;
  WriteLn
end.
