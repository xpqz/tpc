program HeapGetMem;
var p: Pointer;
begin
  WriteLn('before');
  while True do GetMem(p, 60000);
end.
