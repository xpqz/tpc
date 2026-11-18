program SetControl;
type Op = (Add, Sub, Mul, Divide);
var o: Op; i, n: Integer; arith: set of Op;
begin
  arith := [Add, Sub];
  for o := Add to Divide do
    if o in arith then WriteLn('additive ', Ord(o))
    else WriteLn('multiplicative ', Ord(o));
  n := 0;
  i := 0;
  while not (i in [10, 20, 30]) do begin
    i := i + 1;
    n := n + 1;
  end;
  WriteLn(i, ' ', n);
  repeat
    i := i + 1;
  until i in [25..26];
  WriteLn(i)
end.
