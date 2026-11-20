program Colours;
uses Crt;
var
  I: Integer;
  Key: Char;
begin
  ClrScr;
  TextColor(White);
  WriteLn('Turbo Pascal 5.0, compiled to WebAssembly');
  WriteLn;
  for I := 1 to 15 do
  begin
    TextColor(I);
    WriteLn('  ', I:2, '  ', #219#219#219#219#219#219#219#219, '  colour');
  end;
  WriteLn;
  TextColor(Yellow);
  Write('Press a key');
  Key := ReadKey
end.
