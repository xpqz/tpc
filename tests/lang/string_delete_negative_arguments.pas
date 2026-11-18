program StringDeleteNeg;
var s: String;
begin
  s := 'abcdefgh';
  Delete(s, 3, -2);
  WriteLn('[', s, ']');
  Delete(s, -1, 2);
  WriteLn('[', s, ']')
end.
