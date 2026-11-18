program SubrangeStorageSizes;
type A = 1..300; U = -1..200; V = 0..65535; W = -1..40000; X = -129..127; Y = 0..200;
var t1: A; u2: U; v2: V; w2: W; x2: X; y2: Y;
begin
  t1 := 300; u2 := 200; v2 := 65535; w2 := 40000; x2 := -129; y2 := 200;
  WriteLn(t1, ' ', u2, ' ', v2, ' ', w2, ' ', x2, ' ', y2);
  u2 := -1; w2 := -1; WriteLn(u2, ' ', w2, ' ', Pred(y2), Succ(t1))
end.
