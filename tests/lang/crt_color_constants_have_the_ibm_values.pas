program ColorConstants;
uses Crt;
begin
  ClrScr;
  WriteLn(Black, ' ', Blue, ' ', Green, ' ', Cyan, ' ', Red, ' ', Magenta, ' ', Brown, ' ', LightGray);
  Write(DarkGray, ' ', LightBlue, ' ', LightGreen, ' ', LightCyan, ' ');
  WriteLn(LightRed, ' ', LightMagenta, ' ', Yellow, ' ', White);
  WriteLn(Blink);
  TextColor(Crt.Yellow);
  TextBackground(Crt.Cyan);
  WriteLn(TextAttr);
end.
