program IoresultErrorCodesForMisuse;
{ every failure is reported through IOResult; the number is read before anything is printed }
var
  f, g: Text;
  c: Char;
  n: Integer;
  s: string;
procedure Report(what: string);
var
  code: Integer;
begin
  code := IOResult;
  WriteLn(what, ': ', code);
end;
begin
  {$I-}
  Assign(f, 'MISSING.TXT');
  Reset(f);
  Report('reset missing file');
  Assign(f, 'NODIR\MISSING.TXT');
  Reset(f);
  Report('reset in missing directory');
  Rewrite(f);
  Report('rewrite in missing directory');
  Assign(f, 'ONE.TXT');
  Close(f);
  Report('close a file that is not open');
  Reset(f);
  Report('reset existing file');
  Write(f, 'x');
  Report('write to a file open for input');
  Close(f);
  Report('close');
  Rewrite(f);
  Report('rewrite existing file');
  Read(f, c);
  Report('read from a file open for output');
  Close(f);
  Assign(g, 'WORD.TXT');
  Reset(g);
  Read(g, n);
  Report('read a number from a word');
  Close(g);
  Report('close after the failed read');
  Assign(g, 'WORD.TXT');
  Reset(g);
  ReadLn(g, s);
  Report('read a line');
  Close(g);
  Report('close');
end.
