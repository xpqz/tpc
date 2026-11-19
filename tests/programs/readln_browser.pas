program ReadLnBrowser;
var
  Name: String;
  A, B: Integer;
begin
  Write('name? ');
  ReadLn(Name);
  Write('a b? ');
  ReadLn(A, B);
  WriteLn('hello ', Name, ' ', A + B);
end.
