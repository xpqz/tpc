type Rec = record c: Word; end;
var a: array[1..2] of Word; r: Rec; p: ^Word; i: Integer;
begin
  Val('9z', i, a[2]); writeln(i, ' ', a[2]);
  Val('42', i, r.c); writeln(i, ' ', r.c);
  New(p); Val('4 ', i, p^); writeln(i, ' ', p^);
end.
