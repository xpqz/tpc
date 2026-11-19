var f: Text; buf: array[1..10] of Char;
begin SetTextBuf(f, buf[1]); SetTextBuf(f, buf, -1); writeln('ok'); end.
