unit B;
interface
uses C;
function FromB: Integer;
implementation
function FromB: Integer;
begin
  FromB := FromC + 1;
end;
begin
  WriteLn('init B');
end.
