program TextfileEofOnEmptyFile;
var
  f: Text;
  s: string;
  n: Integer;
begin
  Assign(f, 'EMPTY.TXT');
  Reset(f);
  WriteLn(Eof(f), ' ', Eoln(f));
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Read(f, n);
  WriteLn(n);
  Close(f);
  Assign(f, 'NEW.TXT');
  Rewrite(f);
  Close(f);
  Reset(f);
  WriteLn(Eof(f));
  Close(f);
end.
