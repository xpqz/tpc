program HeapNew;
type PBig = ^Big; Big = record next: PBig; data: array[1..1000] of Byte; end;
var head, p: PBig;
begin
  WriteLn('before');
  head := nil;
  while True do begin
    New(p);
    p^.next := head;
    head := p;
  end;
end.
