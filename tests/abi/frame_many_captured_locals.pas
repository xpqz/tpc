program FrameManyCapturedLocals;
{ a frame with twenty captured locals of mixed sizes: distinct offsets, no overlap }
procedure Outer;
var
  a1: Byte; a2: Integer; a3: LongInt; a4: Word; a5: ShortInt;
  b1: Byte; b2: Integer; b3: LongInt; b4: Word; b5: ShortInt;
  c1: Boolean; c2: Char; c3: Integer; c4: LongInt; c5: Byte;
  d1: Word; d2: ShortInt; d3: Integer; d4: LongInt; d5: Boolean;
  procedure Set1;
  begin
    a1 := 1; a2 := -2; a3 := 300000; a4 := 65000; a5 := -5;
    b1 := 255; b2 := 32767; b3 := -300000; b4 := 1; b5 := 127;
    c1 := True; c2 := 'Z'; c3 := -32768; c4 := 2147483647; c5 := 0;
    d1 := 0; d2 := -128; d3 := 5; d4 := -2147483647; d5 := False
  end;
begin
  Set1;
  WriteLn(a1, ' ', a2, ' ', a3, ' ', a4, ' ', a5);
  WriteLn(b1, ' ', b2, ' ', b3, ' ', b4, ' ', b5);
  WriteLn(c1, ' ', c2, ' ', c3, ' ', c4, ' ', c5);
  WriteLn(d1, ' ', d2, ' ', d3, ' ', d4, ' ', d5)
end;
begin
  Outer
end.
