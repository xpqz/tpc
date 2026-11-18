program FrameStringArray;
function Digits(n: Integer): String;
var d: array[0..9] of Char; i: Integer; s: String;
begin
  for i := 0 to 9 do d[i] := Chr(48 + i);
  s := '';
  repeat s := d[n mod 10] + s; n := n div 10 until n = 0;
  Digits := s
end;
begin
  WriteLn(Digits(0), ' ', Digits(907), ' ', Digits(12345))
end.
