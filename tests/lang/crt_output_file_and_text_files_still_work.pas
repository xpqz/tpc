program CrtFiles;
uses Crt;
var
  F: Text;
  S: string;
  N: Integer;
begin
  ClrScr;
  Write(Output, 'to ');
  WriteLn(Output, 'screen');
  Assign(F, 'CRT.TXT');
  Rewrite(F);
  WriteLn(F, 'file line one');
  WriteLn(F, 42);
  Close(F);
  Reset(F);
  ReadLn(F, S);
  ReadLn(F, N);
  Close(F);
  WriteLn(S, ' ', N + 1);
  WriteLn(WhereX, ',', WhereY);
end.
