program WriteToFileUsesTheConsoleFormats;
{ the same items and field widths written to a file read back as written to the screen }
var
  f: Text;
  s: string;
begin
  Assign(f, 'FMT.TXT');
  Rewrite(f);
  Write(f, 'a', 'b':3, 'c');
  WriteLn(f);
  WriteLn(f, 42:6, -7, 'x', True, False:7);
  WriteLn(f, 1.5:8:2, 'z', 'q':0, 'r':-2);
  WriteLn(f, 123456789, ' ', -2147483647, ' ', 65535);
  WriteLn(f, 'ab' + 'cd', #65, 'k');
  Write(f, 'no line end');
  Close(f);
  Assign(f, 'FMT.TXT');
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn('[', s, ']');
  end;
  Close(f);
end.
