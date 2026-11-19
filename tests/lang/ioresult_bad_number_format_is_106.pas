program IoresultBadNumberFormatIs106;
var
  f: Text;
  n: Integer;
  r: Real;
  l: LongInt;
  code: Integer;
  good: Integer;
begin
  {$I-}
  Assign(f, 'BAD.TXT');
  Reset(f);
  good := 0;
  Read(f, n);
  code := IOResult;
  if code = 0 then good := good + 1;
  WriteLn('integer ', n, ' ', code);
  ReadLn(f);
  Read(f, n);
  code := IOResult;
  WriteLn('letters ', code);
  ReadLn(f);
  Read(f, r);
  code := IOResult;
  WriteLn('real ', code);
  ReadLn(f);
  Read(f, n);
  code := IOResult;
  WriteLn('too big ', n, ' ', code);
  ReadLn(f);
  Read(f, l);
  code := IOResult;
  WriteLn('longint ', l, ' ', code);
  ReadLn(f);
  Close(f);
  WriteLn(good);
end.
