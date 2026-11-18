program FuncBasic;
var a, b: Integer;
function Max(a, b: Integer): Integer;
begin
  if a > b then Max := a else Max := b
end;
function Min(a, b: Integer): Integer;
begin
  if a < b then Min := a else Min := b
end;
function Square(n: Integer): LongInt;
var r: LongInt;
begin
  r := n;
  Square := r * n
end;
function Three: Integer;
begin
  Three := 3
end;
begin
  a := 17;
  b := -4;
  WriteLn(Max(a, b), ' ', Min(a, b));
  WriteLn(Max(Min(5, 9), Max(1, 2)));
  WriteLn(Square(200));
  WriteLn(Three + Three * Three)
end.
