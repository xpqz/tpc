program TextModeSetsLastMode;
uses Crt;
var r: array[1..6] of Word; i: Integer;
begin
  TextMode(BW80);   r[1] := LastMode;
  TextMode(Mono);   r[2] := LastMode;
  TextMode(CO80);   r[3] := LastMode;
  TextMode(CO40);   r[4] := LastMode;
  TextMode(BW40);   r[5] := LastMode;
  TextMode(CO80 + Font8x8); r[6] := LastMode;
  TextMode(CO80);
  Writeln(BW40, ' ', CO40, ' ', BW80, ' ', CO80, ' ', Mono, ' ', Font8x8);
  for i := 1 to 6 do Write(r[i], ' ');
  Writeln;
  Writeln(TextAttr, ' ', WindMin, ' ', LastMode);
end.
