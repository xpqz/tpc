program StringLengthByte;
var s: String; t: String[4]; i: Integer;
begin
  s := 'abcde';
  WriteLn(Ord(s[0]), ' ', Length(s));
  s[0] := Chr(3);
  WriteLn(s, ' ', Length(s));
  s[0] := #0;
  WriteLn(Length(s), '[', s, ']');
  t := 'xy';
  t[0] := Chr(4);
  t[3] := 'p'; t[4] := 'q';
  WriteLn(t, ' ', Length(t));
  i := 0;
  s := 'hello';
  while i < Ord(s[0]) do begin i := i + 1; Write(s[i]) end;
  WriteLn
end.
