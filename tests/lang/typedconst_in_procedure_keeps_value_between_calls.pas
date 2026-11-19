program TypedconstInProcedureKeepsValueBetweenCalls;
function NextId: Integer;
const
  last: Integer = 100;
begin
  last := last + 1;
  NextId := last;
end;
procedure Greet;
const
  times: Integer = 0;
  names: array[0..2] of string[5] = ('Ann', 'Bo', 'Cy');
begin
  WriteLn('hello ', names[times mod 3], ' #', times);
  times := times + 1;
end;
var
  i: Integer;
begin
  WriteLn(NextId, ' ', NextId, ' ', NextId);
  for i := 1 to 4 do
    Greet;
end.
