program PendingErrorReadsYieldZeroAndLeaveTheFileAlone;
{$I-}
var
  f, g: Text;
  s: string;
  n, e: Integer;
  c: Char;
  r: Real;
  atEof, atEoln: Boolean;

procedure Fail;
begin
  Assign(f, 'NONE.TXT');
  Reset(f);
end;

begin
  Assign(g, 'G.TXT');
  Reset(g);
  Fail;
  n := 5;
  c := 'x';
  s := 'k';
  r := 1.5;
  atEof := True;
  atEoln := True;
  Read(g, n);
  Read(g, c);
  ReadLn(g, s);
  Read(g, r);
  atEof := Eof(g);
  atEoln := Eoln(g);
  e := IOResult;
  WriteLn(n, ' ', Ord(c), ' [', s, '] ', r:0:1, ' ', atEof, ' ', atEoln, ' ', e);
  ReadLn(g, s);
  WriteLn('[', s, '] ', IOResult);
  ReadLn(g, s);
  WriteLn('[', s, '] ', IOResult);
  Close(g);
end.
