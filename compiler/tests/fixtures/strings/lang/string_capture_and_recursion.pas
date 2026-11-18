program StringCaptureAndRecursion;
var global: String;
procedure Outer(tag: String);
var local: String; ch: String[1];
  procedure Mid(n: Integer);
  var inner: String[5];
    procedure Leaf;
    begin
      local := local + ch;
      inner := inner + tag;
      global := global + '.'
    end;
  begin
    inner := '<';
    Leaf; Leaf;
    local := local + inner;
    if n > 0 then Mid(n - 1)
  end;
begin
  local := tag + ':';
  ch := '*';
  Mid(2);
  WriteLn(local, ' ', Length(local), ' ', global)
end;
function Build(n: Integer): String;
var s: String;
begin
  if n = 0 then Build := '' else
  begin
    s := Build(n - 1);
    Build := s + Chr(Ord('a') + n - 1) + s
  end
end;
function Reverse(s: String): String;
begin
  if Length(s) <= 1 then Reverse := s else Reverse := Reverse(Copy(s, 2, 255)) + s[1]
end;
begin
  global := '';
  Outer('ab');
  Outer('Z');
  WriteLn(Build(4), ' ', Length(Build(7)));
  WriteLn(Reverse('recursion'), Reverse(''), Reverse('x'))
end.
