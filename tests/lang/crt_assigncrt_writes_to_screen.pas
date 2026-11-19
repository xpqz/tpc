program AssignCrtScreen;
uses Crt;
var f: Text; x, y: Byte;
begin
  Assign(f, 'ignored.txt');
  AssignCrt(f);
  Rewrite(f);
  Writeln(f, 'to the screen');
  Write(f, 'abc');
  x := WhereX; y := WhereY;
  Close(f);
  Writeln;
  Writeln(x, ' ', y);
  Window(10, 5, 40, 8);
  TextColor(Yellow);
  Rewrite(f);
  Writeln(f, 'in the window');
  Write(f, 'next');
  Writeln(IOResult);
  Writeln(f, 12, ' ', 3.5:0:1, ' ', True);
  Close(f);
  Writeln(IOResult);
end.
