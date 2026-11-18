program SetElementKinds;
type Color = (Red, Green, Blue, Yellow, Black);
  CSet = set of Color;
  Digit = 0..9;
  DSet = set of Digit;
var c: Color; cs: CSet; b: set of Boolean; d: DSet; i: Integer;
procedure Show(s: CSet);
var k: Color;
begin
  for k := Red to Black do Write(Ord(k in s));
  WriteLn
end;
begin
  cs := [Green, Blue];
  Show(cs);
  Show(cs + [Black] - [Green]);
  Show([Red..Yellow] * [Blue..Black]);
  Show([]);
  Show([Red..Black]);
  c := Yellow;
  cs := [c, Pred(c), Succ(Red)];
  Show(cs);
  WriteLn(Blue in cs, ' ', Black in cs, ' ', [Blue] <= cs, ' ', cs >= [Red, Blue]);
  b := [True];
  WriteLn(True in b, ' ', False in b, ' ', (1 = 1) in b, ' ', (1 = 2) in b);
  b := b + [False];
  WriteLn(True in b, ' ', False in b, ' ', b = [False, True]);
  d := [];
  for i := 0 to 9 do if odd(i) then d := d + [i];
  WriteLn(3 in d, ' ', 4 in d, ' ', d * [3..6] = [3, 5])
end.
