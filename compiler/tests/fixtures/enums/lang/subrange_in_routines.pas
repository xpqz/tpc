program SubrangeInRoutines;
type Small = 1..9; Pct = 0..100;
var s: Small;
function Half(p: Pct): Pct;
begin
  Half := p div 2
end;
procedure Inc2(var x: Small);
begin
  x := x + 2
end;
function Clamp(i: Integer): Pct;
begin
  if i < 0 then Clamp := 0
  else if i > 100 then Clamp := 100
  else Clamp := i
end;
procedure Local;
type Digit = 0..9; Pair = (Lo2, Hi2);
var d: Digit; p: Pair;
begin
  d := 9; p := Hi2;
  WriteLn(d, Ord(p))
end;
begin
  s := 3;
  Inc2(s);
  WriteLn(s, ' ', Half(99), ' ', Half(Clamp(300)), ' ', Clamp(-4), ' ', Clamp(50));
  Local
end.
