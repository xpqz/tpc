program RealLong;
var l: LongInt; r: Real;
begin
  l := 2000000000;
  r := l;
  r := r * 2;
  WriteLn(r:0:0);
  r := l + 1000000000.0;
  WriteLn(r:0:0);
  l := 123456789;
  r := l;
  WriteLn(r:0:0);
  WriteLn(Trunc(r / 1000), ' ', Round(r / 1000));
  r := l * 0.5;
  WriteLn(r:0:1);
  WriteLn(l / 3 > 41152262, ' ', l / 3 < 41152264)
end.
