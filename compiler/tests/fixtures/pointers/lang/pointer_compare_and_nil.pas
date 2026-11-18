program Compare;
type PI = ^Integer;
var a, b, c: PI; g: Pointer; i: Integer;
begin
  WriteLn(a = nil);
  a := nil; b := nil;
  WriteLn(a = b, ' ', a <> b, ' ', nil = a, ' ', a = nil);
  New(a); New(b);
  WriteLn(a = b, ' ', a <> b, ' ', a <> nil, ' ', nil <> b);
  c := a;
  WriteLn(c = a, ' ', c <> b);
  g := a;
  WriteLn(g = a, ' ', a = g, ' ', g <> b, ' ', g = nil);
  c := @i;
  WriteLn(c = @i, ' ', c <> a);
  g := nil;
  c := g;
  WriteLn(c = nil);
  if a = nil then WriteLn('nil') else WriteLn('set');
  Dispose(b); Dispose(a)
end.
