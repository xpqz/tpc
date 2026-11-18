program SetSub;
type
  Small = 1..20;
  Letter = 'a'..'f';
  SS = set of Small;
  LS = set of Letter;
var a, b: SS; l: LS; x: Small; c: Letter; bs: set of Boolean;
begin
  a := [1, 3, 5, 20];
  b := [3..7];
  for x := 1 to 20 do
    if x in a * b then Write(x, ' ');
  WriteLn;
  for x := 1 to 20 do
    if x in a + b then Write(x, ' ');
  WriteLn;
  l := ['a', 'c'..'e'];
  for c := 'a' to 'f' do
    if c in l then Write(c);
  WriteLn;
  bs := [True];
  WriteLn(True in bs, ' ', False in bs);
  bs := bs + [False];
  WriteLn(True in bs, ' ', False in bs, ' ', bs = [False, True])
end.
