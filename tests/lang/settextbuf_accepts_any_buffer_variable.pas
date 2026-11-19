var f: Text; buf: Integer; s: String; size: LongInt;
begin
  size := 100;
  SetTextBuf(f, buf, size);
  SetTextBuf(f, buf, 0);
  SetTextBuf(f, s);
  SetTextBuf(f, s, 80);
  SetTextBuf(f, f);
  writeln('ok');
end.
