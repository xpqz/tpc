program Probe;
var
  Buf: array[0..65534] of Char;
begin
  Buf[65534] := 'x';
  WriteLn('ok ', Buf[65534]);
end.
