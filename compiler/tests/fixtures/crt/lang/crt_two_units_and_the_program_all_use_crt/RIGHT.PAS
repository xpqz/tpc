unit Right;
interface
uses Crt;
procedure ShowRight;
implementation
procedure ShowRight;
begin
  TextBackground(Blue);
  Write('right ');
  TextBackground(Black)
end;
begin
  GotoXY(3, 2)
end.
