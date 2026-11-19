program LongIntIndex;
{ A LongInt (and a Word, and an Integer made from one) as the index of a 64 KB heap array. }
type
  Bytes = array[0..65534] of Byte;
var
  Buf: ^Bytes;
  L: LongInt;
  W: Word;
  Sum: LongInt;
begin
  New(Buf);
  L := 0;
  while L <= 65534 do
  begin
    Buf^[L] := L mod 251;
    Inc(L);
  end;
  Sum := 0;
  for L := 0 to 65534 do Sum := Sum + Buf^[L];
  WriteLn('sum of all bytes by LongInt index: ', Sum);
  L := 40000;
  WriteLn('Buf^[40000] by LongInt: ', Buf^[L]);
  W := 40000;
  WriteLn('Buf^[40000] by Word: ', Buf^[W]);
  L := 65534;
  WriteLn('Buf^[65534] by LongInt: ', Buf^[L]);
  L := 65534;
  WriteLn('Buf^[L - 1] ', Buf^[L - 1], ', Buf^[L + 0] ', Buf^[L + 0]);
  L := 50000;
  W := 20000;
  WriteLn('Buf^[L - W] (LongInt - Word): ', Buf^[L - W]);
end.
