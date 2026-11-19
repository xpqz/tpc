program Video;
uses Crt;
begin
  ClrScr;
  TextAttr := $12;
  WriteLn('start ', TextAttr);
  HighVideo;
  WriteLn('high ', TextAttr);
  LowVideo;
  WriteLn('low ', TextAttr);
  HighVideo;
  HighVideo;
  WriteLn('high twice ', TextAttr);
  NormVideo;
  WriteLn('norm ', TextAttr);
  TextAttr := $E4;
  LowVideo;
  WriteLn('low ', TextAttr);
  NormVideo;
  WriteLn('norm ', TextAttr);
end.
