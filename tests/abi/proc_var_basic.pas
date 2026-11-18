program ProcVarBasic;
{$F+}
{ call routines through procedural variables, with and without parameters and results }
type
  TProc = procedure;
  TFunc = function(a, b: Integer): Integer;
  TNoArg = function: Integer;
var p: TProc; f: TFunc; n: TNoArg;
procedure Hello; begin WriteLn('hello') end;
function Add(a, b: Integer): Integer; begin Add := a + b end;
function Seven: Integer; begin Seven := 7 end;
begin
  p := Hello; f := Add; n := Seven;
  p;
  WriteLn(f(3, 4));
  WriteLn(n + 1);
  WriteLn(f(f(1, 2), n))
end.
