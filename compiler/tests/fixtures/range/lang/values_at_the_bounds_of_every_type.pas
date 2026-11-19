program Bounds;
{$R+}
type Small = 1..10; Letter = 'a'..'e'; Neg = -5..5; Day = (Mon, Tue, Wed); Mid = Tue..Wed;
var b: Byte; sh: ShortInt; i: Integer; w: Word; l: LongInt; s: Small; c: Letter; n: Neg;
    d: Day; m: Mid;
    a: array[-3..3] of Integer; t: String[3]; u: String; j: Integer;
begin
  i := 0;
  b := i; WriteLn(b);
  i := 255;
  b := i; WriteLn(b);
  i := -128;
  sh := i; WriteLn(sh);
  i := 127;
  sh := i; WriteLn(sh);
  l := 65535;
  w := l; WriteLn(w);
  l := 0;
  w := l; WriteLn(w);
  l := 32767;
  i := l; WriteLn(i);
  l := -32768;
  i := l; WriteLn(i);
  i := 1;
  s := i; WriteLn(s);
  i := 10;
  s := i; WriteLn(s);
  i := -5;
  n := i; WriteLn(n);
  i := 5;
  n := i; WriteLn(n);
  c := Chr(Ord('a'));
  WriteLn(c);
  c := Chr(Ord('e'));
  WriteLn(c);
  d := Tue;
  m := d; WriteLn(Ord(m));
  d := Wed;
  m := d; WriteLn(Ord(m));
  for j := -3 to 3 do a[j] := j;
  j := -3; WriteLn(a[j]);
  j := 3; WriteLn(a[j]);
  t := 'xyz';
  for j := 0 to 3 do Write(Ord(t[j]), ' ');
  WriteLn;
  u := 'q';
  j := 255; WriteLn(Ord(u[j]) - Ord(u[j]));
  j := 0; WriteLn(Ord(u[j]))
end.
