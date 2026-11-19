unit Strs;
interface
function Shout(s: string): string;
procedure Append(var s: string; more: string);
function Repeated(s: string; n: Integer): string;
function Join(a, b: string): string;
implementation
function Shout(s: string): string;
var
  i: Integer;
begin
  for i := 1 to Length(s) do
    s[i] := UpCase(s[i]);
  Shout := s + '!';
end;
procedure Append(var s: string; more: string);
begin
  s := s + more;
end;
function Repeated(s: string; n: Integer): string;
var
  r: string;
  i: Integer;
begin
  r := '';
  for i := 1 to n do
    r := r + s;
  Repeated := r;
end;
function Join(a, b: string): string;
begin
  Join := a + '-' + b;
end;
end.
