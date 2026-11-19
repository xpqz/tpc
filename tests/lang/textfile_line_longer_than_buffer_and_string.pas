program TextfileLineLongerThanBufferAndString;
{ a 300 character line: a string takes 255, the rest is read by the next Read }
var
  f: Text;
  a, b: string;
begin
  Assign(f, 'WIDE.TXT');
  Reset(f);
  ReadLn(f, a);
  WriteLn(Length(a));
  WriteLn(a[1], a[255]);
  Close(f);
  Reset(f);
  Read(f, a);
  Read(f, b);
  WriteLn(Length(a), ' ', Length(b));
  WriteLn(b);
  Close(f);
end.
