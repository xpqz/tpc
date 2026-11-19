program UtilsPower;
uses Utils;

procedure Show(B, E: Real);
begin
  WriteLn(B:0:2, ' ^ ', E:0:2, ' = ', Power(B, E):0:4);
end;

begin
  Show(2, 10);
  Show(2, 0);
  Show(2, 0.5);
  Show(10, -2);
  Show(9, 0.5);
  Show(1, 123.5);
  Show(0.5, 3);
  Show(0, 3);
  Show(0, 0.5);
  Show(0, 0);
  Show(-2, 0);
  Show(-2, 3);
  Show(-2, 4);
  Show(-3, 2);
  Show(-3, -3);
  Show(-0.5, 2);
  Show(7, 1);
end.
