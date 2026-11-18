program NegTest;
type T = (A, B); U = (P, Q);
var x: T;
begin
  case x of
    A: WriteLn;
    Q: WriteLn
  end
end.
