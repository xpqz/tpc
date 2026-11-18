program DisposeGlobal;
var g: Integer; p: ^Integer;
begin
  WriteLn('before');
  p := @g;
  p^ := 1;
  Dispose(p);
  WriteLn('not reached')
end.
