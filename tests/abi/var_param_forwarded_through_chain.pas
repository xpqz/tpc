program VarParamForwardedThroughChain;
{ var parameter passed on as a var argument through several levels }
procedure L3(var x: Integer); begin x := x * 2 end;
procedure L2(var x: Integer); begin L3(x); x := x + 1; L3(x) end;
procedure L1(var x: Integer); begin L2(x); x := x - 3; L2(x) end;
var n: Integer;
begin
  n := 1;
  L1(n);
  WriteLn(n)
end.
