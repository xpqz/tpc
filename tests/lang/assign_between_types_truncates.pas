program AssignBetweenTypesTruncates;
var
  b: Byte;
  s: ShortInt;
  i: Integer;
  w: Word;
  l: LongInt;
begin
  i := 300;
  b := i;
  WriteLn(b);
  b := 200;
  s := b;
  WriteLn(s);
  i := -1;
  w := i;
  WriteLn(w);
  w := 65535;
  i := w;
  WriteLn(i);
  l := 70000;
  i := l;
  WriteLn(i);
  l := -70000;
  w := l;
  WriteLn(w);
  i := -5;
  l := i;
  WriteLn(l);
  w := 65535;
  l := w;
  WriteLn(l);
  l := 65535;
  s := l;
  WriteLn(s);
  s := -1;
  b := s;
  WriteLn(b);
  s := -1;
  w := s;
  WriteLn(w)
end.
