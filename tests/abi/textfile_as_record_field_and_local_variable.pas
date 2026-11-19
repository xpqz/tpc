program TextfileAsRecordFieldAndLocalVariable;
type
  TChannel = record
    id: Integer;
    f: Text;
    lines: Integer;
  end;
var
  a, b: TChannel;
procedure Open(var c: TChannel; id: Integer; name: string);
begin
  c.id := id;
  c.lines := 0;
  Assign(c.f, name);
  Rewrite(c.f);
end;
procedure Put(var c: TChannel; s: string);
begin
  WriteLn(c.f, c.id, ':', s);
  c.lines := c.lines + 1;
end;
procedure Dump(name: string);
var
  f: Text;
  s: string;
begin
  Assign(f, name);
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    Write(s, ' ');
  end;
  WriteLn;
  Close(f);
end;
begin
  Open(a, 1, 'A.TMP');
  Open(b, 2, 'B.TMP');
  Put(a, 'x');
  Put(b, 'y');
  Put(a, 'z');
  Close(a.f);
  Close(b.f);
  WriteLn(a.lines, ' ', b.lines);
  Dump('A.TMP');
  Dump('B.TMP');
end.
