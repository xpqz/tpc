program EnumRoutines;
type Color = (Red, Green, Blue); Shade = Color;
var g: Color; s: Shade;
function Next(c: Color): Color;
begin
  if c = Blue then Next := Red else Next := Succ(c)
end;
procedure Bump(var c: Color);
begin
  c := Next(c)
end;
procedure Show(c: Color);
type Local = (One, Two);
var l: Local;
begin
  l := Two;
  WriteLn(Ord(c), Ord(l))
end;
function Count(from, upto: Color): Integer;
var c: Color; n: Integer;
begin
  n := 0;
  for c := from to upto do n := n + 1;
  Count := n
end;
procedure Walk(c: Color);
  procedure Inner;
  begin
    c := Next(c)
  end;
begin
  Inner; Inner;
  WriteLn(Ord(c))
end;
begin
  g := Blue;
  Bump(g); Show(g);
  Bump(g); Show(g);
  s := g; Bump(s);
  WriteLn(Ord(s), Ord(Next(Next(s))), Count(Red, Blue), Count(Green, Green), Count(Blue, Red));
  Walk(Green);
  WriteLn(Next(Next(Next(Red))) = Red)
end.
