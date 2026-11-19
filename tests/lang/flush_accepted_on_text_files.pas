program FlushIsAccepted;
var f: Text; s: string; r: Integer;
begin
  Assign(f, 'out.txt');
  Rewrite(f);
  Writeln(f, 'one');
  Flush(f);
  Writeln(f, 'two');
  Flush(f);
  Flush(f);
  Close(f);
  Writeln(IOResult);
  Reset(f);
  Readln(f, s); Writeln(s);
  Readln(f, s); Writeln(s);
  Close(f);
  Flush(Output);
  Write('shown');
  Flush(Output);
  Writeln;
  Writeln(IOResult);
  Append(f);
  Writeln(f, 'three');
  Flush(f);
  Close(f);
  Reset(f);
  while not Eof(f) do begin Readln(f, s); Writeln(s) end;
  Close(f);
  {$I-}
  Flush(f);
  r := IOResult;
  Writeln('closed ', r);
  Reset(f);
  Flush(f);
  r := IOResult;
  Writeln('input ', r);
  Close(f);
  {$I+}
end.
