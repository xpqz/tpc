program PtrNil;
type PI = ^Integer;
var p, q: PI;
begin
  WriteLn(p = nil, ' ', nil = p, ' ', p <> nil);
  p := nil;
  q := nil;
  WriteLn(p = q);
  New(p);
  WriteLn(p = nil, ' ', p <> nil, ' ', p = q);
  q := p;
  WriteLn(p = q, ' ', p <> q);
  New(q);
  WriteLn(p = q, ' ', p <> q);
  Dispose(q);
  Dispose(p)
end.
