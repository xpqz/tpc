program UnitScreen;
uses Crt, Banner;
begin
  ClrScr;
  Write('main ');
  Show('unit');
  Write(WhereX, ',', WhereY, ' ');
  Corner;
  Write(WhereX, ',', WhereY);
end.
