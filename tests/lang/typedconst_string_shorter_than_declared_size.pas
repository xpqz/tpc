program TypedconstStringShorterThanDeclaredSize;
{ a String[n] constant keeps its capacity: later assignments fill up to n }
const
  s: string[8] = 'ab';
begin
  WriteLn(s, ' ', Length(s));
  s := s + 'cdefghijkl';
  WriteLn(s, ' ', Length(s));
end.
