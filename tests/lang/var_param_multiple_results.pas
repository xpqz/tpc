program VarParamMultipleResults;
{ var parameters as extra results: quotient, remainder and sign }
procedure DivRem(a, b: Integer; var q, r: Integer; var neg: Boolean);
begin
  q := a div b;
  r := a mod b;
  neg := (a < 0) <> (b < 0)
end;
var q, r: Integer; n: Boolean;
begin
  DivRem(17, 5, q, r, n);
  WriteLn(q, ' ', r, ' ', n);
  DivRem(-17, 5, q, r, n);
  WriteLn(q, ' ', r, ' ', n);
  DivRem(-17, -5, q, r, n);
  WriteLn(q, ' ', r, ' ', n)
end.
