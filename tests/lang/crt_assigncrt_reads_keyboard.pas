program AssignCrtKeyboard;
uses Crt;
var f: Text; s: string; n: Integer; c: Char;
begin
  AssignCrt(f);
  Reset(f);
  Readln(f, s);
  Writeln('got ', s);
  Readln(f, n);
  Writeln('num ', n + 1);
  Close(f);
  Rewrite(f);
  Write(f, 'done');
  Close(f);
end.
