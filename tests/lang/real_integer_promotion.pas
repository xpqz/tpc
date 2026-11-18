program RealPromote;
var r: Real; i: Integer; l: LongInt; b: Byte; w: Word;
begin
  i := 7;
  r := i;
  WriteLn(r:0:1);
  r := i + 0.5;
  WriteLn(r:0:1);
  r := i * 0.25;
  WriteLn(r:0:2);
  l := 100000;
  r := l;
  WriteLn(r:0:1);
  r := l * 1.5;
  WriteLn(r:0:1);
  b := 200;
  r := b + 55.5;
  WriteLn(r:0:1);
  w := 65535;
  r := w;
  WriteLn(r:0:1);
  r := w + 1;
  WriteLn(r:0:1);
  r := i / 2;
  WriteLn(r:0:2);
  r := 2 * r + i;
  WriteLn(r:0:2);
  r := i - 10;
  WriteLn(r:0:1);
  r := -l;
  WriteLn(r:0:0);
  r := i + i;
  WriteLn(r:0:0)
end.
