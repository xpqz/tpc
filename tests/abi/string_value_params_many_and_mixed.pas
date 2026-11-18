program StringParamsMixed;
type S3 = String[3];
function F(a: Integer; s: String; b: Byte; t: S3; c: Char; u: String): String;
var r: String;
begin
  r := s + '|' + t + '|' + c + '|' + u + '|';
  s[1] := '#';
  F := r + s
end;
begin
  WriteLn(F(1, 'abc', 2, 'defgh', 'z', 'last'));
  WriteLn(F(-1, '', 255, '', 'q', ''))
end.
