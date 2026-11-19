unit Left;
interface
procedure ShowLeft;
implementation
uses Crt;
procedure ShowLeft;
begin
  TextColor(Yellow);
  Write('left ');
  TextColor(LightGray)
end;
begin
  Window(1, 1, 40, 25)
end.
