var f: Text; buf: array[1..1024] of Char; s: String;
begin
  Assign(f, 'T.TXT'); SetTextBuf(f, buf); Rewrite(f); writeln(f, 'hello'); writeln(f, 'x'); Close(f);
  Assign(f, 'T.TXT'); SetTextBuf(f, buf, 512); Reset(f); ReadLn(f, s); writeln(s); ReadLn(f, s); writeln(s); Close(f);
  writeln('done');
end.
