program Wrap;
{ A 65535-byte block that does not start at offset 0 of its segment: TP indexes with 16-bit
  offsets, so the tail of the block lands back at the start of the segment. }
type
  Bytes = array[0..65534] of Byte;
var
  Small: ^Bytes;
  Big: ^Bytes;
  I: Word;
  Hits: Word;
begin
  GetMem(Small, 100);
  for I := 0 to 99 do Small^[I] := $AA;
  GetMem(Big, 65535);
  WriteLn('same segment: ', Seg(Small^) = Seg(Big^), ' offsets ', Ofs(Small^), ' ', Ofs(Big^));
  for I := 0 to 65534 do Big^[I] := $55;
  Hits := 0;
  for I := 0 to 99 do
    if Small^[I] <> $AA then Inc(Hits);
  WriteLn('sentinel bytes overwritten: ', Hits);
end.
