program ReportMergeTwoFilesIntoThird;
{ merges two sorted files of numbers into one sorted file }
var
  a, b, c: Text;
  x, y: Integer;
  haveX, haveY: Boolean;
  s: string;
  count: Integer;
procedure NextA;
begin
  haveX := not Eof(a);
  if haveX then ReadLn(a, x);
end;
procedure NextB;
begin
  haveY := not Eof(b);
  if haveY then ReadLn(b, y);
end;
begin
  Assign(a, 'A.TXT');
  Assign(b, 'B.TXT');
  Assign(c, 'MERGED.TXT');
  Reset(a);
  Reset(b);
  Rewrite(c);
  NextA;
  NextB;
  while haveX or haveY do
    if haveX and (not haveY or (x <= y)) then
    begin
      WriteLn(c, x);
      NextA;
    end
    else
    begin
      WriteLn(c, y);
      NextB;
    end;
  Close(a);
  Close(b);
  Close(c);
  Reset(c);
  count := 0;
  while not Eof(c) do
  begin
    ReadLn(c, s);
    Write(s, ' ');
    count := count + 1;
  end;
  WriteLn;
  WriteLn(count, ' numbers');
  Close(c);
end.
