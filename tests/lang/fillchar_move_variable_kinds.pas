program FillMoveKinds;
type
  Rec = record
    tag: Byte;
    pair: array[1..2] of Integer;
  end;
var
  a, b: array[1..8] of Byte;
  words: array[1..4] of Integer;
  r: Rec;
  s, t: String;
  w: Word;
  l: LongInt;
  p: ^Rec;
  i: Integer;

procedure InFrame(var out: Rec);
var
  scratch: array[1..4] of Byte;
  n: Integer;
begin
  FillChar(scratch, SizeOf(scratch), 9);
  n := 0;
  for i := 1 to 4 do n := n + scratch[i];
  Move(scratch, out.pair, 4);
  FillChar(out.tag, 1, 255);
  WriteLn('frame ', n, ' ', out.pair[1], ' ', out.pair[2], ' ', out.tag);
end;

procedure ByValue(x: Integer);
begin
  FillChar(x, 2, 1);
  WriteLn('value ', x);
end;

begin
  FillChar(a, SizeOf(a), 7);
  FillChar(a[3], 2, 0);
  FillChar(b, 8, Ord('A'));
  for i := 1 to 8 do Write(a[i], ' ');
  WriteLn;
  WriteLn(Chr(b[1]), Chr(b[8]));
  FillChar(b, 3, 'z');
  WriteLn(Chr(b[1]), Chr(b[3]), Chr(b[4]));
  FillChar(b, 2, True);
  WriteLn(b[1], b[2], b[3]);
  FillChar(words, SizeOf(words), 255);
  WriteLn(words[1], ' ', words[4]);
  FillChar(words[2], 2, 0);
  WriteLn(words[1], ' ', words[2], ' ', words[3]);
  { Value is an ordinal expression; only its low byte is stored. }
  FillChar(a, 4, 3 + 4);
  WriteLn(a[1], a[4], a[5]);
  i := 261;
  FillChar(a, 2, i);
  WriteLn(a[1], a[2]);

  Move(a, b, 8);
  WriteLn(b[1], ' ', b[5], ' ', b[8]);
  Move(a, b, 0);
  Move(words, r.pair, 4);
  WriteLn(r.pair[1], ' ', r.pair[2]);
  r.tag := 1;
  Move(r, r, SizeOf(r));
  WriteLn(r.tag);

  s := 'hello';
  FillChar(t, SizeOf(t), 0);
  WriteLn(Length(t));
  Move(s, t, 6);
  WriteLn(t, Length(t));
  FillChar(s[1], 3, '*');
  WriteLn(s);
  FillChar(s, 1, 2);
  WriteLn(Length(s));

  w := 0;
  FillChar(w, 1, 255);
  WriteLn(w);
  FillChar(w, 2, 255);
  WriteLn(w);
  l := 0;
  FillChar(l, 4, 1);
  WriteLn(l);
  Move(l, w, 2);
  WriteLn(w);
  Move(w, l, 2);
  WriteLn(l);

  New(p);
  FillChar(p^, SizeOf(Rec), 4);
  WriteLn(p^.tag, ' ', p^.pair[2]);
  FillChar(p^.pair, 4, 0);
  Move(p^, r, SizeOf(Rec));
  WriteLn(r.tag, ' ', r.pair[2]);
  Dispose(p);

  with r do
  begin
    FillChar(tag, 1, 9);
    Move(r.tag, pair[2], 1);
  end;
  WriteLn(r.tag, ' ', r.pair[2]);

  InFrame(r);
  ByValue(0);
end.
