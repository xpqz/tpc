program ReportGeneratorReadsDataWritesReportAndReadsItBack;
{ reads SALES.DAT (name quantity price per line), writes REPORT.TXT, then reads REPORT.TXT
  and prints it with line numbers }
const
  MaxItems = 20;
var
  names: array[1..MaxItems] of string[12];
  qty: array[1..MaxItems] of Integer;
  price: array[1..MaxItems] of Real;
  count, i: Integer;
  data, report: Text;
  total, grand: Real;
  line: string;
  lineNo: Integer;
  name: string[12];
  q: Integer;
  p: Real;
begin
  Assign(data, 'SALES.DAT');
  Reset(data);
  count := 0;
  while not Eof(data) do
  begin
    ReadLn(data, name, q, p);
    count := count + 1;
    names[count] := name;
    qty[count] := q;
    price[count] := p;
  end;
  Close(data);
  Assign(report, 'REPORT.TXT');
  Rewrite(report);
  WriteLn(report, 'SALES REPORT');
  WriteLn(report, '============');
  grand := 0;
  for i := 1 to count do
  begin
    total := qty[i] * price[i];
    grand := grand + total;
    WriteLn(report, names[i]:12, qty[i]:5, price[i]:8:2, total:10:2);
  end;
  WriteLn(report, '------------');
  WriteLn(report, 'items: ', count);
  WriteLn(report, 'total: ', grand:0:2);
  Close(report);
  Reset(report);
  lineNo := 0;
  while not Eof(report) do
  begin
    ReadLn(report, line);
    lineNo := lineNo + 1;
    WriteLn(lineNo:2, ' ', line);
  end;
  Close(report);
end.
