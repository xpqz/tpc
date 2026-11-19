program LineEndsOtherThanCrLf;
{ only CR ends a line for ReadLn and Eoln; a LF alone is an ordinary character }
var
  f: Text;
  s: string;
  c: Char;
  n: Integer;
begin
  Assign(f, 'LFONLY.TXT');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    WriteLn(n, ' length ', Length(s), ' first ', Ord(s[1]), ' last ', Ord(s[Length(s)]));
  end;
  Close(f);
  Assign(f, 'LONECR.TXT');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    WriteLn(n, ' [', s, ']');
  end;
  Close(f);
  Assign(f, 'LFONLY.TXT');
  Reset(f);
  Read(f, c);
  Read(f, c);
  Read(f, c);
  Read(f, c);
  WriteLn(Ord(c), ' ', Eoln(f));
  Close(f);
end.
