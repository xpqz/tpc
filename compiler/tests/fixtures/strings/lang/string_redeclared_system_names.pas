program StringRedeclaredSystemNames;
var a, b: String;
function Length(s: String): Integer;
begin
  Length := 42
end;
function Pos(sub, s: String): Integer;
begin
  Pos := -1
end;
function Copy(s: String; i, n: Integer): String;
begin
  Copy := 'copy'
end;
function Concat(x, y: String): String;
begin
  Concat := 'concat'
end;
procedure Delete(var s: String; i, n: Integer);
begin
  s := 'deleted'
end;
procedure Insert(src: String; var s: String; i: Integer);
begin
  s := 'inserted'
end;
function UpCase(c: Char): Char;
begin
  UpCase := '!'
end;
begin
  a := 'abc'; b := 'xyz';
  WriteLn(a + b, ' ', a < b, ' ', a = b, ' ', a + 'd' + b);
  WriteLn(Length(a), ' ', Pos('b', a), ' ', Copy(a, 1, 1), ' ', Concat(a, b), ' ', UpCase('a'));
  Delete(a, 1, 1);
  WriteLn(a);
  Insert(b, a, 1);
  WriteLn(a, b[1], b[2] + b[3]);
  a := 'abc';
  WriteLn(a + b, a[2], a[0] = #3)
end.
