program SubrangeCaseAndFor;
type Grade = 0..100; Level = 1..3;
var g: Grade; lv: Level; n: Integer;
function Letter(x: Grade): Char;
begin
  case x of
    90..100: Letter := 'A';
    80..89: Letter := 'B';
    0..79: Letter := 'C'
  end
end;
begin
  g := 95; Write(Letter(g));
  g := 80; Write(Letter(g));
  g := 3; WriteLn(Letter(g));
  for lv := 1 to 3 do
  begin
    case lv of
      1: Write('one ');
      2: Write('two ');
      3: Write('three ')
    end
  end;
  WriteLn;
  n := 0;
  for g := 98 to 100 do n := n + g;
  for g := 2 downto 0 do n := n + g;
  WriteLn(n);
  lv := 2;
  case lv + 1 of
    3: WriteLn('three');
    4: WriteLn('four')
  end
end.
