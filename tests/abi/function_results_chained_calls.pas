program FunctionResultsChainedCalls;
{ results of calls used as arguments of further calls and in mixed expressions }
function Add(a, b: Integer): Integer; begin Add := a + b end;
function Mul(a, b: LongInt): LongInt; begin Mul := a * b end;
function Neg(a: Integer): Integer; begin Neg := -a end;
function Pick(c: Boolean; a, b: Integer): Integer;
begin if c then Pick := a else Pick := b end;
begin
  WriteLn(Add(Add(1, 2), Add(3, Add(4, 5))));
  WriteLn(Mul(Add(100, 200), Mul(Add(1, 2), 1000)));
  WriteLn(Neg(Neg(Neg(5))), ' ', Add(Neg(3), Neg(4)));
  WriteLn(Pick(Add(1, 1) = 2, Neg(1), 99), ' ', Pick(Add(1, 1) = 3, 1, Neg(99)))
end.
