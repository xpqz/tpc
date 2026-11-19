program PendingErrorDoesNotStopOpenCloseAndAssign;
{ Reset, Rewrite, Append and Close run although an error is pending, and their own failure
  replaces it; Read, Write, Eof and Eoln do nothing }
{$I-}
var
  f, g, h: Text;
  s: string;
  e: Integer;

procedure Fail;
begin
  Assign(f, 'NONE.TXT');
  Reset(f);
end;

begin
  Assign(g, 'G.TXT');
  Rewrite(g);
  WriteLn(g, 'first');
  Fail;
  WriteLn(g, 'skipped');
  e := IOResult;
  WriteLn('write: ', e);
  Fail;
  Close(g);
  e := IOResult;
  WriteLn('close: ', e);
  Fail;
  Reset(g);
  e := IOResult;
  WriteLn('reset: ', e);
  ReadLn(g, s);
  WriteLn('[', s, ']');
  Fail;
  Assign(h, 'H.TXT');
  Rewrite(h);
  WriteLn(h, 'written');
  Close(h);
  e := IOResult;
  WriteLn('rewrite: ', e);
  Reset(h);
  ReadLn(h, s);
  WriteLn('[', s, '] ', IOResult);
  Close(h);
  Fail;
  Close(f);
  e := IOResult;
  WriteLn('close of the unopened file: ', e);
  Close(g);
  Close(g);
  e := IOResult;
  WriteLn('second close: ', e);
end.
