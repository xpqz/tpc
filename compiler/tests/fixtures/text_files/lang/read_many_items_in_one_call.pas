program ReadManyItemsInOneCall;
var
  f: Text;
  b: Byte;
  sh: ShortInt;
  w: Word;
  l: LongInt;
  r: Real;
  c: Char;
  s: string;
begin
  Assign(f, 'MIX.TXT');
  Reset(f);
  Read(f, b, sh, w, l, r, c, s);
  WriteLn(b, ' ', sh, ' ', w, ' ', l, ' ', r:6:1, '[', c, ']', s);
  ReadLn(f, c);
  WriteLn(Ord(c));
  Read(f, l);
  WriteLn(l, ' ', Eof(f));
  Close(f);
end.
