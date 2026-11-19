unit Ops;
{$F+}
interface
uses Callback;
var
  Table: array[0..1] of TBinary;
function Add(a, b: Integer): Integer;
function Mul(a, b: Integer): Integer;
function Sub(a, b: Integer): Integer;
function Fold(op: TBinary; first, last: Integer): Integer;
procedure EachUpTo(n: Integer; visit: TVisit);
procedure Pick(addition: Boolean; var f: TBinary);
implementation
function Add(a, b: Integer): Integer;
begin
  Add := a + b;
end;
function Mul(a, b: Integer): Integer;
begin
  Mul := a * b;
end;
function Sub(a, b: Integer): Integer;
begin
  Sub := a - b;
end;
function Fold(op: TBinary; first, last: Integer): Integer;
var
  i, acc: Integer;
begin
  acc := first;
  for i := first + 1 to last do
    acc := op(acc, i);
  Fold := acc;
end;
procedure EachUpTo(n: Integer; visit: TVisit);
var
  i: Integer;
begin
  for i := 1 to n do
    visit(i);
end;
procedure Pick(addition: Boolean; var f: TBinary);
begin
  if addition then f := Add else f := Sub;
end;
begin
  Table[0] := Add;
  Table[1] := Mul;
end.
