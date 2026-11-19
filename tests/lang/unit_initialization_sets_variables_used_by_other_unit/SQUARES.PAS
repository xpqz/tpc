unit Squares;
interface
function SquareOf(n: Integer): Integer;
implementation
uses Table;
function SquareOf(n: Integer): Integer;
begin
  if n < Filled then SquareOf := Values[n] else SquareOf := n * n;
end;
end.
