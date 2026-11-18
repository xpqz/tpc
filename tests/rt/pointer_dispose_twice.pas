program DisposeTwice;
var p: ^Integer;
begin
  WriteLn('before');
  New(p);
  Dispose(p);
  Dispose(p);
  WriteLn('not reached')
end.
