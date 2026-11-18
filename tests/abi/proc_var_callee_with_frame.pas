program ProcVarCalleeWithFrame;
{$F+}
{ routines called through variables that have frames (var arguments, Exit, nested routines):
  the stack pointer is restored after every indirect call }
type TStep = procedure(var x: Integer; by: Integer);
var step: TStep; trace: Integer;
procedure Bump(var x: Integer); begin x := x + 1 end;
procedure Plain(var x: Integer; by: Integer);
var t: Integer;
begin
  t := by;
  Bump(t);
  x := x + t
end;
procedure Early(var x: Integer; by: Integer);
var t: Integer;
begin
  t := 0;
  Bump(t);
  if by > 5 then Exit;
  x := x + by + t
end;
procedure Nested(var x: Integer; by: Integer);
var acc: Integer;
  procedure Add;
  begin
    acc := acc + by
  end;
begin
  acc := x;
  Add; Add;
  x := acc
end;
procedure Drive(p: TStep; var x: Integer);
var i: Integer;
begin
  for i := 1 to 3 do p(x, i * 3)
end;
var n: Integer;
begin
  n := 0;
  step := Plain; Drive(step, n); WriteLn(n);
  step := Early; Drive(step, n); WriteLn(n);
  step := Nested; Drive(step, n); WriteLn(n)
end.
