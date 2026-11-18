program SetParams;
type TS = set of 0..9;
procedure Add(s: TS; x: Integer);
begin
  s := s + [x];
  WriteLn(x in s);
end;
procedure AddVar(var s: TS; x: Integer);
begin
  s := s + [x];
end;
function Count(s: TS): Integer;
var k, n: Integer;
begin
  n := 0;
  for k := 0 to 9 do
    if k in s then n := n + 1;
  Count := n;
end;
procedure MakeEvens(var r: TS);
var k: Integer;
begin
  r := [];
  for k := 0 to 9 do
    if not Odd(k) then r := r + [k];
end;
var a, ev: TS;
begin
  a := [1, 2];
  Add(a, 7);
  WriteLn(7 in a, ' ', Count(a));
  AddVar(a, 7);
  WriteLn(7 in a, ' ', Count(a));
  MakeEvens(ev);
  WriteLn(Count(ev), ' ', Count(ev + [1, 3]), ' ', 4 in ev, ' ', ev * a = [2]);
  a := ev;
  WriteLn(Count(a), ' ', a = ev)
end.
