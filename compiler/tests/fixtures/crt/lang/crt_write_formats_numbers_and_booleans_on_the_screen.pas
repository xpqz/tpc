program Formats;
uses Crt;
var r: Real;
    l: LongInt;
begin
  ClrScr;
  TextColor(Yellow);
  TextBackground(Blue);
  r := 3.14159;
  l := -123456;
  WriteLn(1, ' ', -7:5, ' ', l, ' ', True, ' ', 'x':4);
  WriteLn(r:8:3, ' ', 'abc', 'd');
  Window(40, 10, 60, 14);
  TextColor(White);
  WriteLn(l:12, True:6, 'tail-that-wraps-around');
  Write(65535, ' ', Ord('A'));
end.
