program TextModeClearsWholeScreen;
uses Crt;
var i: Integer;
begin
  for i := 1 to 24 do Writeln('row ', i);
  TextColor(White);
  TextBackground(Red);
  ClrScr;
  Write('red');
  TextMode(CO80);
  Write('plain');
  Window(1, 1, 80, 25);
  Writeln;
  Writeln(WhereY);
end.
