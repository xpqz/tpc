program PtrPtr;
type PI = ^Integer; PPI = ^PI;
var i: Integer; p: PI; pp: PPI;
begin
  i := 3;
  p := @i;
  pp := @p;
  WriteLn(pp^^);
  pp^^ := 4;
  WriteLn(i);
  New(pp);
  New(pp^);
  pp^^ := 77;
  WriteLn(pp^^);
  Dispose(pp^);
  Dispose(pp)
end.
