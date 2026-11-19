program Fits;
{$R+}
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun); Work = Mon..Fri; Small = 1..10;
var b: Byte; w: Word; i: Integer; l: LongInt; sh: ShortInt; s: Small;
    d: Day; k: Work; c: Char;
    ab: array[Byte] of Integer; ad: array[Day] of Integer; abool: array[Boolean] of Integer;
    ak: array[Work] of Integer; ac: array[Char] of Byte; as1: array[Small] of Integer;
procedure V(var x: Byte);
begin
  x := 200;
end;
begin
  b := 255;
  w := b;
  i := b;
  l := w;
  l := i;
  sh := 127;
  i := sh;
  l := sh;
  s := 10;
  i := s;
  b := s;
  d := Sun;
  d := Pred(d);
  k := Fri;
  d := k;
  b := 255;
  ab[b] := 1;
  ab[0] := 2;
  WriteLn(ab[255], ab[0]);
  ad[d] := 3;
  ad[k] := 4;
  ak[k] := 5;
  abool[True] := 6;
  abool[False] := 7;
  WriteLn(ad[Fri], ak[Fri], abool[True], abool[False]);
  c := Chr(255);
  ac[c] := 8;
  ac[Chr(0)] := 9;
  WriteLn(ac[Chr(255)], ac[Chr(0)]);
  as1[s] := 10;
  WriteLn(as1[10]);
  V(b);
  WriteLn(b);
  l := 40000;
  w := 65535;
  l := w;
  WriteLn(l)
end.
