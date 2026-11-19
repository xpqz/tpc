program MaxIntConstants;
const
  Limit = MaxInt;
  Twice = MaxInt * 2;
  Longest = MaxLongInt;
var
  a: array[1..MaxInt] of Byte;
  i: Integer;
  l: LongInt;
  w: Word;
begin
  WriteLn(MaxInt, ' ', MaxLongInt);
  WriteLn(Limit, ' ', Twice, ' ', Longest);
  WriteLn(SizeOf(a));
  WriteLn(MaxInt + 1, ' ', MaxInt * 2, ' ', MaxLongInt - 1, ' ', -MaxInt - 1);
  i := MaxInt;
  l := i;
  l := l + 1;
  WriteLn(i, ' ', l);
  w := MaxInt;
  w := w + w;
  WriteLn(w);
  l := MaxLongInt;
  WriteLn(l, ' ', l div MaxInt, ' ', l mod MaxInt);
  for i := MaxInt - 2 to MaxInt do Write(i, ' ');
  WriteLn;
  WriteLn(Ord(MaxInt = 32767), Ord(MaxLongInt = 2147483647), Ord(MaxLongInt > MaxInt));
end.
