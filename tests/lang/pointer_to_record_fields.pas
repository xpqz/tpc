program PtrRec;
type
  PRec = ^Rec;
  Rec = record
    id: Integer;
    ch: Char;
    flag: Boolean;
    name: String[10];
  end;
var p, q: PRec;
begin
  New(p);
  p^.id := 7;
  p^.ch := 'x';
  p^.flag := True;
  p^.name := 'record';
  WriteLn(p^.id, ' ', p^.ch, ' ', p^.flag, ' ', p^.name);
  q := p;
  q^.id := q^.id + 1;
  WriteLn(p^.id);
  with p^ do begin
    id := id * 10;
    name := name + '!';
  end;
  WriteLn(q^.id, ' ', q^.name);
  New(q);
  q^ := p^;
  q^.name := 'copy';
  WriteLn(p^.name, ' ', q^.name, ' ', q^.id);
  Dispose(q);
  Dispose(p)
end.
