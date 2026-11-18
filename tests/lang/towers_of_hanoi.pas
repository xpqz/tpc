program TowersOfHanoi;
var moves: Integer;
procedure Hanoi(n: Integer; from, to_, via: Char);
begin
  if n > 0 then
  begin
    Hanoi(n - 1, from, via, to_);
    moves := moves + 1;
    if n <= 3 then WriteLn('move disk ', n, ' from ', from, ' to ', to_);
    Hanoi(n - 1, via, to_, from)
  end
end;
begin
  moves := 0;
  Hanoi(3, 'A', 'C', 'B');
  WriteLn(moves, ' moves');
  moves := 0;
  Hanoi(10, 'A', 'C', 'B');
  WriteLn(moves, ' moves')
end.
