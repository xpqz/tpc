program DerefForms;
type
  PR = ^Rec; PA = ^Arr; PPRec = ^PR;
  Rec = record a: Integer; b: array[1..3] of Byte; s: String[5]; next: PR end;
  Arr = array[0..4] of Rec;
var p, q: PR; arrp: PA; pp: PPRec; i: Integer;
begin
  New(p); New(q);
  p^.a := 1; p^.b[2] := 22; p^.s := 'abc'; p^.next := q;
  q^.a := 2; q^.next := nil;
  WriteLn(p^.a, ' ', p^.b[2], ' ', p^.s, ' ', p^.next^.a);
  WriteLn(p^.next^.next = nil);
  pp := @p;
  WriteLn(pp^^.a, ' ', pp^^.next^.a, ' ', pp^^.s[2]);
  pp^^.a := 10;
  WriteLn(p^.a);
  with p^ do begin a := a + 5; s := s + 'd'; b[3] := 33 end;
  WriteLn(p^.a, ' ', p^.s, ' ', p^.b[3]);
  New(arrp);
  for i := 0 to 4 do begin arrp^[i].a := i * i; arrp^[i].s := 'x' end;
  for i := 0 to 4 do Write(arrp^[i].a, arrp^[i].s, ' ');
  WriteLn;
  arrp^[2] := p^;
  WriteLn(arrp^[2].a, ' ', arrp^[2].s, ' ', arrp^[2].next^.a);
  p^ := arrp^[1];
  WriteLn(p^.a, ' ', p^.s);
  Dispose(arrp); Dispose(q); Dispose(p)
end.
