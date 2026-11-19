program AddrOfProductPlusOne;
var a: Integer; p: Pointer;
begin
  p := Addr(a * 2 + 1);
end.
