program FuncResultTypes;
function B: Byte;
begin
  B := 255
end;
function S: ShortInt;
begin
  S := -128
end;
function W: Word;
begin
  W := 65535
end;
function L: LongInt;
begin
  L := -2147483647 - 1
end;
function Bo(n: Integer): Boolean;
begin
  Bo := n > 0
end;
function C(n: Integer): Char;
begin
  C := Chr(n)
end;
begin
  WriteLn(B, ' ', S, ' ', W, ' ', L);
  WriteLn(Bo(3), ' ', Bo(-3), ' ', C(65), C(66));
  WriteLn(B + 1, ' ', W - 1)
end.
