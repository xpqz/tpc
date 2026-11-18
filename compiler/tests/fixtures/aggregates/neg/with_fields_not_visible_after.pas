program P;
type Rec = record a, b: Integer end;
var rv: Rec;
begin
  with rv do begin a := 1; b := a + 1 end;
  a := 3
end.
