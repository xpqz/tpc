program TextModeResets;
uses Crt;
var attr, wmin, wmax, last, x, y: Word;
begin
  Writeln('first line');
  Writeln('second line');
  Window(10, 5, 40, 12);
  TextColor(Yellow);
  TextBackground(Blue);
  GotoXY(3, 3);
  Write('inside window');
  TextMode(CO80);
  attr := TextAttr; wmin := WindMin; wmax := WindMax; last := LastMode;
  x := WhereX; y := WhereY;
  Writeln('attr=', attr, ' min=', wmin, ' max=', wmax, ' last=', last, ' at ', x, ',', y);
  Write('after');
end.
