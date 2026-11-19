program NegTest;
{$F+}
type
  TFun = function(x: Integer): Integer;
  TRec = record
    f: TFun;
  end;
var r: TRec;
function Id(x: Integer): Integer;
begin
  Id := x
end;
begin
  r.f := Id;
  r.f(1)
end.
