program SetTemps;
type BS = set of 0..63;
var a: BS; i, n, depth: Integer;
function Pick(x: Integer): Boolean;
begin
  Pick := (x in [1, 3, 5, 7..9]) or (x in [20..30] - [25]) or (x in [40] + [41] + [42, 43])
end;
function Fib(k: Integer; s: BS): Integer;
var t: BS;
begin
  t := s + [k];
  if k < 2 then
    Fib := k + Ord(k in t)
  else
    Fib := Fib(k - 1, t + [k]) + Fib(k - 2, t * [0..k]) + Ord([k] <= t) - Ord([k + 1] <= t)
end;
procedure Deep(d: Integer; s: BS);
var local: BS;
begin
  local := s + [d];
  if d < 20 then Deep(d + 1, local + [d + 1] - [d div 2]);
  if (d in s) or (d in local) and ([d] + [d + 1] <= local + s + [d + 1]) then n := n + 1
end;
begin
  n := 0;
  for i := 0 to 63 do if Pick(i) then n := n + 1;
  WriteLn(n);
  for i := 0 to 12 do Write(Pick(i), ' ');
  WriteLn;
  WriteLn(Fib(10, []), ' ', Fib(5, [1, 2]));
  n := 0; Deep(0, []);
  WriteLn(n);
  a := [1, 2] + [3] + [4, 5] + [6..8] + [9] + [10, 11, 12] + [13] + [14, 15, 16] + [17] + [18];
  n := 0;
  for i := 0 to 63 do if i in a then n := n + 1;
  WriteLn(n)
end.
