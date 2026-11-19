program ClrScrAttr;
uses Crt;
begin
  Write('plain');
  TextBackground(Blue);
  TextColor(Yellow);
  ClrScr;
  Write('on blue');
  GotoXY(1, 25);
  Write('last row');
end.
