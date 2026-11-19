program AddrOfFunctionPlusOne;
var p: Pointer;
function G: Integer;
begin
  G := 1;
end;
begin
  p := Addr(G + 1);
end.
