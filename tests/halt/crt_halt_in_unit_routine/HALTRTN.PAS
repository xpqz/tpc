unit HaltRtn;
interface
procedure Stop(code: Word);
implementation
procedure Stop(code: Word);
begin
  WriteLn('stopping with ', code);
  Halt(code)
end;
end.
