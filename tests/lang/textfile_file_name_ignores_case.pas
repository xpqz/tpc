program TextfileFileNameIgnoresCase;
var
  f: Text;
  s: string;
begin
  Assign(f, 'data.txt');
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
  Assign(f, 'Data.Txt');
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
  Assign(f, 'made.tmp');
  Rewrite(f);
  WriteLn(f, 'created in lower case');
  Close(f);
  Assign(f, 'MADE.TMP');
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
end.
