program IoresultReadingAtEndOfFileIsNotAnError;
var
  f: Text;
  n: Integer;
  r: Real;
  s: string;
  code: Integer;
begin
  {$I-}
  Assign(f, 'ONE.TXT');
  Reset(f);
  ReadLn(f, s);
  WriteLn(Eof(f));
  ReadLn(f, s);
  Read(f, n);
  Read(f, r);
  ReadLn(f);
  ReadLn(f, s);
  code := IOResult;
  WriteLn('[', s, '] ', n, ' ', r:0:1, ' ', code);
  Close(f);
end.
