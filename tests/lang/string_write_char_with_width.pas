program CharWriteWidth;
var c: Char;
begin
  c := 'k';
  WriteLn('[', c:1, '][', c:4, '][', c:0, ']');
  WriteLn('[', 'z':3, ']')
end.
