program NegTest;
type RT = record a: Integer end;
var r: RT;
begin
  with r do a := 1;
  a := 2
end.
