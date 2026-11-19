program LocalConstantsAreStatic;
function Counter: Integer;
const
  calls: Integer = 0;
begin
  calls := calls + 1;
  Counter := calls;
end;
procedure Outer;
const
  depth: Integer = 10;
  names: array[1..2] of string[3] = ('one', 'two');
  procedure Inner;
  const
    inner: Integer = 100;
  begin
    inner := inner + depth;
    depth := depth + 1;
    WriteLn('inner ', inner, ' ', names[depth mod 2 + 1]);
  end;
begin
  Inner;
  Inner;
  WriteLn('outer ', depth);
end;
procedure Same;
const
  calls: Integer = 50;
begin
  calls := calls + 1;
  WriteLn('same ', calls);
end;
begin
  WriteLn(Counter, ' ', Counter, ' ', Counter);
  Outer;
  Outer;
  Same;
  WriteLn(Counter);
  Same;
end.
