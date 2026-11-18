program ParamConversions;
var
  b: Byte;
  i: Integer;
  w: Word;
procedure PL(l: LongInt);
begin
  WriteLn('L ', l)
end;
procedure PI(i: Integer);
begin
  WriteLn('I ', i)
end;
procedure PB(b: Byte);
begin
  WriteLn('B ', b)
end;
procedure PW(w: Word);
begin
  WriteLn('W ', w)
end;
begin
  b := 200;
  i := -5;
  w := 65535;
  PL(b);
  PL(i);
  PL(w);
  PI(b);
  PW(b);
  i := 300;
  PB(i);
  i := -1;
  PW(i);
  PB(i);
  w := 65535;
  PI(w)
end.
