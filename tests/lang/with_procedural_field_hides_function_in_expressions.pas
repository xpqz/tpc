program WithProceduralFieldHidesFunction;
{$F+}
type
  Op = function(x: Integer): Integer;
  Rec = record
    Twice: Op;
    Level: Integer;
  end;
var
  r: Rec;

function Twice(x: Integer): Integer;
begin
  Twice := -x
end;

function Triple(x: Integer): Integer;
begin
  Triple := 3 * x
end;

function Double(x: Integer): Integer;
begin
  Double := 2 * x
end;

begin
  r.Twice := Triple;
  r.Level := 4;
  WriteLn(Twice(5));
  with r do
  begin
    WriteLn(Twice(5));
    WriteLn(Twice(Level) + 1);
    Level := Twice(Twice(2));
    WriteLn(Level)
  end;
  r.Twice := Double;
  with r do
    WriteLn(Twice(21))
end.
