program UnitTypedConstantsAndVariablesKeepState;
uses Ticker;
begin
  WriteLn(Tick, ' ', Tick, ' ', Tick);
  WriteLn(Total);
  Restart;
  WriteLn(Tick);
  WriteLn(Names[1], Names[3]);
  Names[2] := 'X';
  WriteLn(Names[2]);
  WriteLn(Origin.x, ',', Origin.y);
  Origin.x := 9;
  Shift;
  WriteLn(Origin.x, ',', Origin.y);
  WriteLn(Greeting, '!');
end.
