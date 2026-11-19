program TextfileReadStringStopsAtEndOfLine;
{ Read of a string takes the rest of the line but not the line end; ReadLn takes the end too }
var
  f: Text;
  s: string;
begin
  Assign(f, 'S.TXT');
  Reset(f);
  Read(f, s);
  WriteLn('[', s, ']');
  Read(f, s);
  WriteLn('[', s, ']');
  ReadLn(f, s);
  WriteLn('[', s, ']');
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Close(f);
end.
