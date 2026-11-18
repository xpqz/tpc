program DisposeNilLink;
type PN = ^N; N = record v: Integer; next: PN end;
var h, p: PN; i: Integer;
procedure FreeAll(p: PN);
var q: PN;
begin
  repeat
    WriteLn('free ', p^.v);
    q := p^.next;
    Dispose(p);
    p := q
  until p = nil;
  Dispose(p)
end;
begin
  h := nil;
  for i := 1 to 3 do begin New(p); p^.v := i; p^.next := h; h := p end;
  FreeAll(h)
end.
