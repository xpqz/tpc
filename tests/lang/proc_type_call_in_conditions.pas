program ProcTypeCallInConditions;
{$F+}
{ calls through variables as operands of if, while, repeat, case and the short-circuit
  operators; a parameterless function variable without parentheses }
type TPred = function(x: Integer): Boolean;
type TNum = function: Integer;
var even: TPred; next: TNum; n: Integer;
function IsEven(x: Integer): Boolean; begin IsEven := not Odd(x) end;
function Counter: Integer; begin n := n + 1; Counter := n end;
begin
  even := IsEven; next := Counter;
  n := 0;
  while next < 4 do Write(n, ' ');
  WriteLn;
  repeat until next >= 8;
  WriteLn(n);
  if even(n) and (next = 9) then WriteLn('yes') else WriteLn('no');
  if even(3) or (next > 100) then WriteLn('or') else WriteLn('nor');
  case next of
    10: WriteLn('ten');
    11: WriteLn('eleven')
  else WriteLn('other')
  end
end.
