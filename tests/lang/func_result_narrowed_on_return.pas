program FuncResultNarrowedOnReturn;
var i: Integer;
function ToByte(n: Integer): Byte;
begin
  ToByte := n
end;
function ToShort(n: Integer): ShortInt;
begin
  ToShort := n
end;
function ToInt(n: LongInt): Integer;
begin
  ToInt := n
end;
begin
  i := 300;
  WriteLn(ToByte(i));
  i := 200;
  WriteLn(ToShort(i));
  WriteLn(ToInt(70000));
  WriteLn(ToInt(-70000))
end.
