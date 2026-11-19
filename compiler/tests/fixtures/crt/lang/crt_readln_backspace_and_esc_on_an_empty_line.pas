program EmptyEdits;
uses Crt;
var s: string;
begin
  ClrScr;
  Write('> ');
  ReadLn(s);
  WriteLn;
  Write('[', s, ']');
  Write(' ', Length(s));
end.
