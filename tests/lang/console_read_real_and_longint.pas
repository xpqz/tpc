program ConsoleReadRealAndLongint;
var
  r: Real;
  l: LongInt;
begin
  ReadLn(r, l);
  WriteLn(r:0:3, ' ', l);
  ReadLn(l);
  WriteLn(l div 7);
  ReadLn(r);
  WriteLn(r * 2:0:2);
end.
