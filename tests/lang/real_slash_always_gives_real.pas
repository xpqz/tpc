program RealSlash;
var i, j: Integer; l: LongInt; r: Real;
begin
  i := 7;
  j := 2;
  r := i / j;
  WriteLn(r:0:3);
  WriteLn(i / j:0:4);
  WriteLn(1 / 4:0:2, ' ', 1 / 8:0:3, ' ', 100 / 8:0:3, ' ', -7 / 2:0:1);
  l := 1000000;
  WriteLn(l / 4:0:2);
  WriteLn(i / 7:0:1, ' ', 0 / i:0:1);
  WriteLn(i div j, ' ', i mod j);
  WriteLn(i / j * j:0:1)
end.
