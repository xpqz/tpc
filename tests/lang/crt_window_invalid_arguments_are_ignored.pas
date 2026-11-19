program WindowInvalid;
uses Crt;
begin
  ClrScr;
  Window(10, 5, 30, 10);
  Write('1');
  Window(0, 5, 30, 10);
  Write('2');
  Window(10, 0, 30, 10);
  Write('3');
  Window(10, 5, 81, 10);
  Write('4');
  Window(10, 5, 30, 26);
  Write('5');
  Window(20, 5, 10, 10);
  Write('6');
  Window(10, 8, 30, 7);
  Write('7');
  Window(31, 11, 40, 11);
  Write('8');
  Window(5, 12, 5, 12);
  Write('9');
  Write('a');
  Window(1, 1, 80, 25);
  GotoXY(1, 20);
  Write('end');
end.
