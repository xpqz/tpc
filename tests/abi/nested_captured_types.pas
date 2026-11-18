program NestedCapturedTypes;
{ captured locals of every scalar type keep their width through a nested routine }
procedure Outer;
var b: Byte; s: ShortInt; i: Integer; w: Word; l: LongInt; t: Boolean; c: Char;
  procedure Bump;
  begin
    b := b + 1; s := s + 1; i := i + 1; w := w + 1; l := l + 1;
    t := not t;
    c := Chr(Ord(c) + 1)
  end;
begin
  b := 255; s := 127; i := 32767; w := 65535; l := 2147483647; t := False; c := 'a';
  Bump;
  WriteLn(b, ' ', s, ' ', i, ' ', w, ' ', l, ' ', t, ' ', c);
  Bump;
  WriteLn(b, ' ', s, ' ', i, ' ', w, ' ', l, ' ', t, ' ', c)
end;
begin
  Outer
end.
