program StringCompare;
procedure T(a, b: String);
begin
  WriteLn(a, ' ', b, ' ', a = b, ' ', a <> b, ' ', a < b, ' ', a > b, ' ', a <= b, ' ', a >= b)
end;
begin
  T('abc', 'abd');
  T('abc', 'abc');
  T('abc', 'ab');
  T('', 'a');
  T('', '');
  T('B', 'a');
  T('a', 'B');
  T('Z', 'a');
  T('10', '9');
  T('abc', 'abcd');
  T('a ', 'a');
  T(#1, '');
  T('a', #1'a')
end.
