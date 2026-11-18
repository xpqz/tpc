program StrInt;
var s: String; i: Integer; l: LongInt; b: Byte;
begin
  Str(123, s);
  WriteLn('[', s, ']');
  Str(-45, s);
  WriteLn('[', s, ']');
  Str(0, s);
  WriteLn('[', s, ']');
  i := 32767;
  Str(i, s);
  WriteLn('[', s, ']', Length(s));
  i := -32768;
  Str(i, s);
  WriteLn('[', s, ']', Length(s));
  l := 2147483647;
  Str(l, s);
  WriteLn('[', s, ']');
  l := -2147483647;
  Str(l - 1, s);
  WriteLn('[', s, ']');
  b := 200;
  Str(b, s);
  WriteLn('[', s, ']');
  Str(5:4, s);
  WriteLn('[', s, ']');
  Str(-5:4, s);
  WriteLn('[', s, ']');
  Str(12345:3, s);
  WriteLn('[', s, ']');
  Str(7:0, s);
  WriteLn('[', s, ']');
  Str(i * 0 + 99:6, s);
  WriteLn('[', s, ']')
end.
