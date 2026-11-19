program ConsoleReadIntegersAndSum;
var
  count, i, n, sum: Integer;
begin
  ReadLn(count);
  sum := 0;
  for i := 1 to count do
  begin
    Read(n);
    sum := sum + n;
  end;
  ReadLn;
  WriteLn('sum of ', count, ' numbers: ', sum);
  ReadLn(n);
  WriteLn(n * 2);
end.
