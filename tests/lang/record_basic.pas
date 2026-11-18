program RecordBasic;
type PT = record x, y: Integer; tag: Char; ok: Boolean end;
var p, q: PT;
begin
  p.x := 10; p.y := -20; p.tag := 'Z'; p.ok := True;
  WriteLn(p.x, ' ', p.y, ' ', p.tag, ' ', p.ok);
  q := p;
  q.x := q.x + 1;
  WriteLn(p.x, ' ', q.x, ' ', q.y, ' ', q.tag);
  WriteLn(SizeOf(PT), ' ', SizeOf(p.x))
end.
