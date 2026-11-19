program Probe;
type Buf = array[0..65535] of Char;
var Q: ^Buf;
begin
  New(Q);
end.
