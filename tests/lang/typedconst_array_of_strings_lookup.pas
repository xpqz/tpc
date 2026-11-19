program TypedconstArrayOfStringsLookup;
const
  words: array[1..4] of string = ('zero', 'one', 'two', 'three');
  months: array[1..12] of string[3] = ('Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
    'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec');
var
  i: Integer;
begin
  for i := 1 to 4 do
    Write(words[i], ' ');
  WriteLn;
  for i := 1 to 12 do
    Write(months[i]);
  WriteLn;
  words[2] := 'ONE';
  WriteLn(words[2], Length(words[2]));
end.
