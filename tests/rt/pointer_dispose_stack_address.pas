program DisposeStack;
var p: ^Integer; i: Integer;
begin
  WriteLn('before');
  p := @i;
  Dispose(p);
  WriteLn('not reached')
end.
