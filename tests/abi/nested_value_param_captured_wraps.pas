program NestedValueParamCapturedWraps;
{ value parameters captured by a nested routine live in the outer frame at their own width:
  copied in at entry, wrapped when the nested routine writes them, the caller's variable
  untouched }
procedure Outer(b: Byte; i: Integer; w: Word; s: ShortInt);
  procedure Grow;
  begin
    b := b + 1; i := i + 1; w := w + 1; s := s + 1
  end;
begin
  Grow;
  WriteLn(b, ' ', i, ' ', w, ' ', s);
  Grow;
  WriteLn(b, ' ', i, ' ', w, ' ', s)
end;
var b: Byte; i: Integer; w: Word; s: ShortInt;
begin
  b := 255; i := 32767; w := 65535; s := 127;
  Outer(b, i, w, s);
  WriteLn(b, ' ', i, ' ', w, ' ', s);
  Outer(0, -1, 0, -1)
end.
