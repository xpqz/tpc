program WithNested;
type Inn = record a, b: Integer end; Outr = record i: Inn; c: Integer end;
var o: Outr; p: Inn;
begin
  with o do begin
    c := 1;
    with i do begin a := 10; b := 20 end;
    WriteLn(c, ' ', i.a, ' ', i.b)
  end;
  with o, i do begin a := a + c; b := b + c; c := 100 end;
  WriteLn(o.i.a, ' ', o.i.b, ' ', o.c);
  p.a := 1; p.b := 2;
  with o do with p do begin a := a + 100; WriteLn(a, ' ', b, ' ', c, ' ', i.a) end;
  WriteLn(p.a, ' ', o.i.a)
end.
