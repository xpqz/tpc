program StringCopyNeg;
var s: String;
begin
  s := 'abcdef';
  WriteLn('[', Copy(s, 2, -1), ']');
  WriteLn('[', Copy(s, 2, -100), ']');
  WriteLn('[', Copy(s, -1, 3), ']')
end.
