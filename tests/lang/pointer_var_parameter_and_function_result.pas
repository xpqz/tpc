program PtrParams;
type PI = ^Integer;
procedure Alloc(var p: PI; v: Integer);
begin
  New(p);
  p^ := v;
end;
function Make(v: Integer): PI;
var p: PI;
begin
  New(p);
  p^ := v;
  Make := p;
end;
procedure Bump(p: PI);
begin
  p^ := p^ + 100;
end;
procedure Release1(var p: PI);
begin
  Dispose(p);
  p := nil;
end;
var a, b: PI;
begin
  Alloc(a, 5);
  b := Make(6);
  WriteLn(a^, ' ', b^);
  Bump(a);
  Bump(b);
  WriteLn(a^, ' ', b^);
  Release1(a);
  Release1(b);
  WriteLn(a = nil, ' ', b = nil)
end.
