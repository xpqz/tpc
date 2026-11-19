program Colors;
uses Crt;
var
  C: Integer;
begin
  ClrScr;
  WriteLn(TextAttr);
  for C := 0 to 15 do
  begin
    TextColor(C);
    Write('c', C, ' ');
  end;
  WriteLn;
  TextColor(White);
  for C := 0 to 7 do
  begin
    TextBackground(C);
    Write('b', C, ' ');
  end;
  WriteLn;
  NormVideo;
  TextColor(Green + Blink);
  Write('blink');
  NormVideo;
  WriteLn(' ', TextAttr);
  TextAttr := $4F;
  Write('attr');
  TextAttr := 7;
  WriteLn;
  TextBackground(Red + 3);
  WriteLn(TextAttr);
end.
