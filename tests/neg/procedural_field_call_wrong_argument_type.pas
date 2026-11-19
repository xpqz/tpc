program NegTest;
{$F+}
type
  TShow = procedure(n: Integer);
  PRec = ^TRec;
  TRec = record
    show: TShow;
  end;
var p: PRec;
procedure Show(n: Integer);
begin
  WriteLn(n)
end;
begin
  New(p);
  p^.show := Show;
  p^.show('x')
end.
