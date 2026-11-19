program WindowCrash;
uses Crt;
var
  Zero: Integer;
begin
  ClrScr;
  Window(10, 5, 40, 10);
  TextAttr := $1E;
  Write('in window ');
  GotoXY(3, 4);
  Write('deeper');
  Zero := 0;
  Write(10 div Zero);
  Write('not reached');
end.
