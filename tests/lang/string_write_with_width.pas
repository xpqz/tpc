program StringWriteWidth;
var s: String; t: String[3];
begin
  s := 'abc'; t := 'xy';
  WriteLn('[', s:6, ']');
  WriteLn('[', s:3, ']');
  WriteLn('[', s:2, ']');
  WriteLn('[', s:1, ']');
  WriteLn('[', s:0, ']');
  WriteLn('[', t:5, ']');
  WriteLn('[', '':4, ']');
  WriteLn('[', '':0, ']');
  s := '';
  WriteLn('[', s:3, ']');
  WriteLn('[', 'ab':7, 'cd':3, ']');
  WriteLn('[', Copy('hello', 2, 3):6, ']');
  WriteLn('[', s + 'q':2, ']')
end.
