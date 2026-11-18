program NegTest;
procedure Two(var a: Integer; var b: Integer);
begin
  a := b
end;
var x: Integer;
begin
  Two(x, 3)
end.
