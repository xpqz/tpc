unit Banner;
interface
procedure Show(S: string);
procedure Corner;
implementation
uses Crt;
procedure Show(S: string);
begin
  TextColor(Yellow);
  Write('<', S, '> ');
  NormVideo;
end;
procedure Corner;
begin
  GotoXY(30, 18);
  Write('corner');
end;
begin
  Window(5, 3, 70, 22);
end.
