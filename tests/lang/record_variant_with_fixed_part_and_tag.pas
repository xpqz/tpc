program VariantTag;
type
  Kind = (KInt, KPair, KChars);
  Item = packed record
    id: Byte;
    case k: Kind of
      KInt: (n: Integer);
      KPair: (a, b: Byte);
      KChars: (c: array[1..4] of Char)
  end;
var it: Item; i: Integer;
begin
  WriteLn(SizeOf(Item));
  it.id := 5; it.k := KChars;
  for i := 1 to 4 do it.c[i] := Chr(64 + i);
  WriteLn(it.id, ' ', Ord(it.k), ' ', it.c[1], it.c[2], it.c[3], it.c[4]);
  it.k := KPair; it.a := 65; it.b := 66;
  WriteLn(it.c[1], it.c[2], ' ', Ord(it.k));
  it.k := KInt; it.n := $4241;
  WriteLn(it.c[1], it.c[2], ' ', it.a, ' ', it.b)
end.
