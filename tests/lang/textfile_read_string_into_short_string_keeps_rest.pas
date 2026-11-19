program TextfileReadStringIntoShortStringKeepsRest;
{ a String[n] takes at most n characters; the rest of the line stays for the next read }
var
  f: Text;
  s: string[5];
  t: string;
begin
  Assign(f, 'LONG.TXT');
  Reset(f);
  Read(f, s);
  WriteLn('[', s, ']');
  Read(f, s);
  WriteLn('[', s, ']');
  ReadLn(f, t);
  WriteLn('[', t, ']');
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Close(f);
end.
