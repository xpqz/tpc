program ProcFieldElementCallStatements;
{$F+}
{ procedure-typed array elements, record fields, nested fields and pointer targets as statements }
type
  TProc = procedure;
  TShow = procedure(n: Integer);
  TPair = procedure(a: Integer; b: Integer);
  TInner = record
    act: TShow;
  end;
  TMenu = record
    first: TProc;
    show: TShow;
    inner: TInner;
    row: array[1..2] of TProc;
  end;
  PMenu = ^TMenu;
var
  table: array[1..3] of TShow;
  steps: array[0..1] of TProc;
  grid: array[1..2, 1..2] of TShow;
  pairs: array[1..2] of TPair;
  m: TMenu;
  p: PMenu;
  i: Integer;

procedure Hello;
begin
  WriteLn('hello')
end;
procedure Bye;
begin
  WriteLn('bye')
end;
procedure Show(n: Integer);
begin
  WriteLn('show ', n)
end;
procedure Twice(n: Integer);
begin
  WriteLn('twice ', n * 2)
end;
procedure Sum(a: Integer; b: Integer);
begin
  WriteLn('sum ', a + b)
end;

begin
  table[1] := Show;
  table[2] := Twice;
  table[3] := Show;
  for i := 1 to 3 do
    table[i](i * 10);
  steps[0] := Hello;
  steps[1] := Bye;
  steps[0];
  steps[1];
  steps[1 - 1];
  grid[2, 1] := Twice;
  grid[2, 1](7);
  grid[2][1](8);
  pairs[2] := Sum;
  pairs[2](3, 4);
  m.first := Hello;
  m.show := Twice;
  m.inner.act := Show;
  m.row[1] := Bye;
  m.row[2] := Hello;
  m.first;
  m.show(21);
  m.inner.act(5);
  m.row[1];
  i := 3;
  m.row[i - 2];
  New(p);
  p^.first := Bye;
  p^.show := Show;
  p^.inner.act := Twice;
  p^.row[2] := Hello;
  p^.first;
  p^.show(99);
  p^.inner.act(6);
  p^.row[2];
  with m do
  begin
    show(1);
    inner.act(2);
    row[2]
  end;
  with p^ do
    first;
  Dispose(p)
end.
