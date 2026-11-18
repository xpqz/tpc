program DisposeWild;
type PI = ^Integer;
var p: PI;
begin
  WriteLn('before');
  p := PI(1);
  Dispose(p);
  WriteLn('not reached')
end.
