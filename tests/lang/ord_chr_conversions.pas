program OrdChrConversions;
var
  c: Char;
  b: Byte;
  i: Integer;
begin
  c := 'A';
  WriteLn(Ord(c));
  b := 97;
  WriteLn(Chr(b));
  WriteLn(Ord('0'), ' ', Ord('9'), ' ', Ord(' '));
  i := 66;
  WriteLn(Chr(i), Chr(i + 1));
  WriteLn(Ord(True), ' ', Ord(False));
  WriteLn(Chr(Ord('a') + 2));
  WriteLn(Ord(Chr(255)))
end.
