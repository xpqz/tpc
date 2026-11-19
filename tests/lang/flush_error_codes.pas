program FlushErrorCodes;
var f, g: Text; r: Integer;
begin
  {$I-}
  Flush(f);
  r := IOResult; Writeln('unassigned ', r);
  Assign(g, 'x.txt');
  Flush(g);
  r := IOResult; Writeln('assigned closed ', r);
  Rewrite(g);
  Close(g);
  Flush(g);
  r := IOResult; Writeln('after close ', r);
  Assign(f, '');
  Reset(f);
  Flush(f);
  r := IOResult; Writeln('console input ', r);
  Rewrite(f);
  Flush(f);
  r := IOResult; Writeln('console output ', r);
  Flush(Input);
  r := IOResult; Writeln('Input ', r);
  {$I+}
end.
