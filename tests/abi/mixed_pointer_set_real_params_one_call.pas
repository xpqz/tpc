program MixedParams;
type PI = ^Integer; CS = set of 0..9;
function Mix(p: PI; s: CS; x: Real; i: Integer; t: String; q: PI): Real;
var k, n: Integer;
begin
  n := 0;
  for k := 0 to 9 do
    if k in s then n := n + 1;
  Mix := p^ + q^ + n + x + i + Length(t);
end;
var a, b: PI; s: CS; r: Real;
begin
  New(a); New(b);
  a^ := 1; b^ := 2;
  s := [1, 2, 3];
  r := Mix(a, s, 0.5, 10, 'abc', b);
  WriteLn(r:0:1);
  r := Mix(b, s + [9], r, -3, '', a);
  WriteLn(r:0:1);
  Dispose(a); Dispose(b)
end.
