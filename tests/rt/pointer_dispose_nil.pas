program DisposeNil;
var p: ^Integer;
begin
  WriteLn('before');
  p := nil;
  Dispose(p);
  WriteLn('not reached')
end.
