program PtrBasic;
var p: ^Integer;
begin
  New(p);
  p^ := 42;
  WriteLn(p^);
  p^ := p^ + 1;
  WriteLn(p^);
  p^ := p^ * 2;
  WriteLn(p^);
  Dispose(p);
  p := nil;
  WriteLn(p = nil)
end.
