program DivModTruncateTowardZero;
procedure Show(a, b: Integer);
begin
  WriteLn(a:4, ' div ', b:2, ' = ', a div b:3, '   ', a:4, ' mod ', b:2, ' = ', a mod b:3)
end;
begin
  Show(7, 2);
  Show(-7, 2);
  Show(7, -2);
  Show(-7, -2);
  Show(6, 3);
  Show(-6, 3);
  Show(1, 5);
  Show(-1, 5);
  Show(0, 5);
  Show(-32768, 1);
  Show(-32768, 7);
  Show(32767, -32767)
end.
