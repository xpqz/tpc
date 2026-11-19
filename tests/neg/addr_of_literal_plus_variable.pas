program AddrOfLiteralPlusVariable;
var a: Integer; p: Pointer;
begin
  p := Addr(1 + a);
end.
