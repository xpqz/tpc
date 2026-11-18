program SetCtor;
var s: set of 0..15; i: Integer;
procedure Show;
var k: Integer;
begin
  for k := 0 to 15 do
    if k in s then Write(k, ' ');
  WriteLn;
end;
begin
  s := [];
  Show;
  s := [3];
  Show;
  s := [1, 3, 5];
  Show;
  s := [2..6];
  Show;
  s := [0, 2..4, 7, 9..11, 15];
  Show;
  s := [5..5];
  Show;
  s := [0..15];
  Show;
  s := [4, 4, 4, 3..5];
  Show;
  i := 8;
  s := [i, i + 1, i - 8, i + 7];
  Show;
  s := [i - 3..i + 2];
  Show;
  s := [i..i - 1];
  Show
end.
