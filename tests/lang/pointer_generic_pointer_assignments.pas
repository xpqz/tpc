program PtrGeneric;
type PI = ^Integer; PR = ^Real;
var g: Pointer; pi1, pi2: PI; r: Real; pr1: PR; i: Integer;
begin
  i := 5;
  g := @i;
  pi1 := g;
  WriteLn(pi1^);
  pi1^ := 6;
  WriteLn(i);
  pi2 := PI(g);
  WriteLn(pi2 = pi1);
  r := 2.5;
  g := @r;
  pr1 := g;
  WriteLn(pr1^:5:2);
  g := nil;
  WriteLn(g = nil);
  New(pi1);
  g := pi1;
  WriteLn(g = pi1, ' ', pi1 = g, ' ', g <> nil);
  Dispose(PI(g))
end.
