program Chains;
type
  PA = ^A; PB = ^B;
  A = record v: Integer; b: PB end;
  B = record w: Integer; a: PA; link: PB end;
  PNode = ^Node;
  Node = record key: Integer; left, right: PNode end;
var x: PA; y: PB; z: PB; t: PNode;
begin
  New(x); New(y); New(z);
  x^.v := 1; x^.b := y;
  y^.w := 2; y^.a := x; y^.link := z;
  z^.w := 3; z^.a := nil; z^.link := nil;
  WriteLn(x^.b^.w, ' ', x^.b^.a^.v, ' ', x^.b^.link^.w, ' ', x^.b^.link^.a = nil);
  New(t);
  t^.key := 5; t^.left := nil; t^.right := nil;
  New(t^.left); t^.left^.key := 3; t^.left^.left := nil; t^.left^.right := nil;
  WriteLn(t^.left^.key, ' ', t^.right = nil);
  Dispose(t^.left); Dispose(t); Dispose(z); Dispose(y); Dispose(x)
end.
