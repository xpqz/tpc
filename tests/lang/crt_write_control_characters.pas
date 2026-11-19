program Controls;
uses Crt;
begin
  ClrScr;
  Write('abcdef');
  Write(#8#8);
  Write('X');
  Write(WhereX);
  WriteLn;
  Write('hello');
  Write(#13);
  Write('HE');
  WriteLn;
  Write('one', #10, 'two');
  WriteLn;
  Write('bell', #7, 'ring');
  WriteLn(' ', WhereX, ',', WhereY);
  Write(#8);
  Write('back');
  GotoXY(1, 8);
  Write(#8, 'wall');
  WriteLn;
  Write('tab', #9, 'x');
  WriteLn;
  Write(#1, #2, #3, #27, #127, #128, #255);
  WriteLn(' ', WhereX);
end.
