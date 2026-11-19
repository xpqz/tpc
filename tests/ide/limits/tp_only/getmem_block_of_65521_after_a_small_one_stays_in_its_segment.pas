program Wrap;
{ The documented largest block, 65521 bytes (64K - $F): the offset of a heap block is at most 15,
  so the block never wraps in its segment. }
type
  Bytes = array[0..65520] of Byte;
var
  Small: ^Bytes;
  Big: ^Bytes;
  I: Word;
  Hits: Word;
begin
  GetMem(Small, 100);
  for I := 0 to 99 do Small^[I] := $AA;
  GetMem(Big, 65521);
  WriteLn('same segment: ', Seg(Small^) = Seg(Big^), ' offsets ', Ofs(Small^), ' ', Ofs(Big^));
  for I := 0 to 65520 do Big^[I] := $55;
  Hits := 0;
  for I := 0 to 99 do
    if Small^[I] <> $AA then Inc(Hits);
  WriteLn('sentinel bytes overwritten: ', Hits);
end.
