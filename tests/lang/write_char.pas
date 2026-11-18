program WriteChar;
var c: Char;
begin
  c := 'A';
  Write(c);
  c := 'z';
  Write(c);
  c := '0';
  Write(c);
  c := ' ';
  Write(c);
  c := '''';
  Write(c);
  WriteLn;
  WriteLn('x', 'y');
  WriteLn('q':3, 'r')
end.
