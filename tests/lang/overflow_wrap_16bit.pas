program OverflowWrap16bit;
var
  i, k: Integer;
  w: Word;
  seed: Word;
begin
  i := 0;
  for k := 1 to 300 do i := i + k;
  WriteLn(i);
  w := 1;
  for k := 1 to 20 do w := w * 3;
  WriteLn(w);
  seed := 12345;
  for k := 1 to 8 do
  begin
    seed := seed * 25173 + 13849;
    Write(seed, ' ')
  end;
  WriteLn;
  i := 1;
  for k := 1 to 20 do
  begin
    i := i * 2;
    Write(i, ' ')
  end;
  WriteLn
end.
