program RangeOk;
{$R+}
type Small = 1..10; Letter = 'a'..'e';
var s: Small; l: Letter; b: Byte; sh: ShortInt; a: array[1..5] of Integer;
    i, k: Integer; w: Word; arr0: array[0..255] of Byte; m: array[-3..3] of Integer;
begin
  s := 1;
  WriteLn(s);
  s := 10;
  WriteLn(s);
  l := 'a';
  l := 'e';
  WriteLn(l);
  b := 0;
  b := 255;
  WriteLn(b);
  sh := -128;
  sh := 127;
  WriteLn(sh);
  i := 200;
  b := i + 55;
  WriteLn(b);
  w := 65535;
  WriteLn(w);
  for k := 1 to 5 do a[k] := k;
  a[1] := a[5];
  k := 1;
  a[k + 3] := a[k];
  WriteLn(a[4]);
  for k := 0 to 255 do arr0[k] := k;
  WriteLn(arr0[255]);
  for k := -3 to 3 do m[k] := k * k;
  WriteLn(m[-3], ' ', m[3]);
  i := 5;
  s := i;
  WriteLn(s);
  s := Succ(s);
  s := s - 1;
  WriteLn(s)
end.
