program TextfileCtrlZEndsTheText;
{ a ^Z byte is the end of a text file; later bytes are never seen }
var
  f: Text;
  s: string;
  c: Char;
begin
  Assign(f, 'EOFMARK.TXT');
  Reset(f);
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Read(f, c);
  WriteLn(c);
  WriteLn(Eoln(f), ' ', Eof(f));
  ReadLn(f, s);
  WriteLn('[', s, ']', Eof(f));
  Close(f);
end.
