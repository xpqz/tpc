program LargestBlock;
{ The largest block GetMem gives (a Word size: 65535) and a byte at both of its ends. }
type
  Bytes = array[0..65534] of Byte;
var
  P: ^Bytes;
  Before: LongInt;
begin
  Before := MemAvail;
  GetMem(P, 65535);
  WriteLn('block of 65535 bytes, heap shrank by at least that: ', Before - MemAvail >= 65535);
  P^[0] := 1;
  P^[65534] := 2;
  WriteLn(P^[0], ' ', P^[65534]);
  FreeMem(P, 65535);
  WriteLn('freed: ', Before - MemAvail);
end.
