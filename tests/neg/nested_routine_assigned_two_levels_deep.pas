program NegTest;
{$F+}
type TProc = procedure;
var p: TProc;
procedure A;
  procedure B;
    procedure C;
    begin
    end;
  begin
    p := C
  end;
begin
  B
end;
begin
  A
end.
