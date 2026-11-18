program ProcNoParams;
var count: Integer;
procedure Hello;
begin
  WriteLn('hello')
end;
procedure Bump;
begin
  count := count + 1
end;
begin
  Hello;
  Hello;
  count := 0;
  Bump;
  Bump;
  Bump;
  WriteLn(count)
end.
