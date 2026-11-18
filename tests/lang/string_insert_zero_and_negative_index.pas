program StringInsertZero;
var s: String;
begin
  s := 'abcdef';
  Insert('XY', s, 0);
  WriteLn('[', s, ']');
  s := 'abcdef';
  Insert('XY', s, -3);
  WriteLn('[', s, ']')
end.
