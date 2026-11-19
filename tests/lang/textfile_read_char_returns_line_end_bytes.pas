program TextfileReadCharReturnsLineEndBytes;
{ Read(c) delivers the CR and LF bytes of a line end; Eoln is true when the next byte is CR }
var
  f: Text;
  c: Char;
begin
  Assign(f, 'TINY.TXT');
  Reset(f);
  while not Eof(f) do
  begin
    Read(f, c);
    if c = #13 then
      WriteLn('CR eoln=', Eoln(f), ' eof=', Eof(f))
    else if c = #10 then
      WriteLn('LF eoln=', Eoln(f), ' eof=', Eof(f))
    else
      WriteLn(c, ' eoln=', Eoln(f), ' eof=', Eof(f));
  end;
  Close(f);
end.
