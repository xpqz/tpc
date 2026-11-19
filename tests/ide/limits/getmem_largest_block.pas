program LargestBlock;
{ The largest block the Reference Guide allows, 65521 bytes (64K - $F), allocated after a small
  one and filled to its last byte: the small block must come out untouched. }
type
  Bytes = array[0..65520] of Byte;
var
  Small: ^Bytes;
  Big: ^Bytes;
  I: Word;
  Hits: Word;
  Before: LongInt;
begin
  GetMem(Small, 100);
  for I := 0 to 99 do Small^[I] := $AA;
  Before := MemAvail;
  GetMem(Big, 65521);
  WriteLn('heap shrank by at least 65521: ', Before - MemAvail >= 65521);
  for I := 0 to 65520 do Big^[I] := $55;
  Hits := 0;
  for I := 0 to 99 do
    if Small^[I] <> $AA then Inc(Hits);
  WriteLn('sentinel bytes overwritten: ', Hits);
  WriteLn(Big^[0], ' ', Big^[65520]);
  FreeMem(Big, 65521);
  WriteLn('freed: ', Before - MemAvail);
end.
