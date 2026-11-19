program UnitStringsPassAcrossUnits;
uses Strs;
var
  s: string;
  t: string[5];
begin
  s := 'hello';
  WriteLn(Shout(s));
  WriteLn(s);
  Append(s, ' world');
  WriteLn(s);
  t := 'abcde';
  Append(s, t);
  WriteLn(s, ' ', Length(s));
  WriteLn(Repeated('ab', 3));
  WriteLn(Repeated('', 3) = '');
  WriteLn(Join('x', 'y') + Join('1', '2'));
end.
