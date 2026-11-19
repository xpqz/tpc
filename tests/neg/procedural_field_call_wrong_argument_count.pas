program NegTest;
{$F+}
type
  TShow = procedure(n: Integer);
  TRec = record
    show: TShow;
  end;
var r: TRec;
procedure Show(n: Integer);
begin
  WriteLn(n)
end;
begin
  r.show := Show;
  r.show(1, 2)
end.
