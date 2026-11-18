program WithBasic;
type PT = record x, y: Integer; n: String[6] end;
var p: PT;
begin
  with p do begin
    x := 3; y := 4; n := 'pt';
    WriteLn(x, ' ', y, ' ', n);
    x := x * y
  end;
  WriteLn(p.x);
  with p do WriteLn(x + y)
end.
