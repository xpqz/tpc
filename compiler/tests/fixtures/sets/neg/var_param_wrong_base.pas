program NegTest;
type A = set of Char; B = set of Byte; var s: A; procedure P(var t: B); begin end;
begin
  P(s);
end.
