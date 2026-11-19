program TextfileWriteFormats;
type
  TColor = (Red, Green);
var
  f: Text;
  s: string;
  r: Real;
begin
  Assign(f, 'FMT.TXT');
  Rewrite(f);
  Write(f, 1, 2, 3);
  WriteLn(f);
  WriteLn(f, 42:6, -7:4, 123456:2);
  WriteLn(f, 'abc':6, 'x', 'toolong':3);
  WriteLn(f, 'c', 'h':3, #65);
  WriteLn(f, True, ' ', False, ' ', True:7);
  r := 3.5;
  WriteLn(f, r:8:2, r:0:0, ' ', -r:6:1);
  WriteLn(f, 100000 * 3, ' ', 65535, ' ', -32768);
  Close(f);
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn('[', s, ']');
  end;
  Close(f);
end.
